use mqtt::ConnectOptions;
use qmetaobject::{prelude::*, queued_callback};

use paho_mqtt as mqtt;
use std::{process, time::Duration};

use rand::distributions::{Alphanumeric, DistString};

use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct MQTTConnection {
    // Define "base" - class
    base: qt_base_class!(trait QObject),

    // Properties
    // the name of the server
    name: qt_property!(QString; NOTIFY nameChanged),
    // the topic to subscribe to
    topic: qt_property!(QString; NOTIFY topicChanged),
    address: qt_property!(QString; NOTIFY addressChanged),
    port: qt_property!(QString; NOTIFY portChanged),
    enableSSL: qt_property!(bool; NOTIFY enableSSLChanged),
    trustSelfSignedCert: qt_property!(bool; NOTIFY trustSelfSignedCertChanged),
    username: qt_property!(QString; NOTIFY usernameChanged),
    password: qt_property!(QString; NOTIFY passwordChanged),

    // Signals
    nameChanged: qt_signal!(),
    topicChanged: qt_signal!(),
    addressChanged: qt_signal!(),
    portChanged: qt_signal!(),
    enableSSLChanged: qt_signal!(),
    trustSelfSignedCertChanged: qt_signal!(),
    usernameChanged: qt_signal!(),
    passwordChanged: qt_signal!(),

    messageArrived: qt_signal!(topic : QString, message : QString),

    // Methods
    connectClient: qt_method!(fn(&mut self)),
    disconnectClient: qt_method!(fn(&mut self)),

    cli: Arc<Mutex<Option<mqtt::AsyncClient>>>,
    worker_chan: Option<Sender<i32>>,

    connected: bool,
    connecting: bool,
}

#[allow(non_snake_case)]
impl MQTTConnection {
    pub fn disconnectClient(&mut self) {
        println!("Disconnecting client");
        self.worker_chan.as_ref().unwrap().send(0);

        drop(self.worker_chan.as_ref().unwrap());

        self.cli.lock().unwrap().as_ref().unwrap().disconnect(None);
        self.connecting = false;
        self.connected = false;
    }

    pub fn connectClient(&mut self) {
        if self.connected == true || self.connecting {
            return;
        }
        println!("Connecting");

        if self.worker_chan.is_some() {
            drop(self.worker_chan.as_ref().unwrap());
        }
        let (tx, rx) = channel::<i32>();

        self.worker_chan = Some(tx);

        self.connecting = true;
        self.connect_worker(rx);
    }

    pub fn connect_worker(&mut self, mut rx: Receiver<i32>) {
        // Nothing to do
        let host = if self.enableSSL == true {
            "ssl://"
        } else {
            "tcp://"
        };
        let host = String::from(host) + &self.address.to_string();
        let host = host + ":" + &self.port.to_string();
        println!("Connecting to the MQTT server at '{}'...", host);

        let clientid = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        let create_opts = mqtt::CreateOptionsBuilder::new_v3()
            .server_uri(host)
            .client_id(String::from("mqtt-plasmoid-") + &clientid)
            .finalize();

        // Create the client connection
        let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
            println!("Error creating the client: {:?}", e);
            process::exit(1);
        });

        // pointer to the qml object to move into the callback
        let qptr = QPointer::from(&*self);

        let on_message_arrived = queued_callback(move |msg_opt: Option<mqtt::Message>| {
            qptr.as_pinned().map(|this| {
                if let Some(msg) = msg_opt {
                    //println!("Message arrived {}, {}", msg.topic(), msg.payload_str());

                    this.borrow_mut().messageArrived(
                        QString::from(msg.topic()),
                        QString::from(msg.payload_str().to_string()),
                    );
                }
            });
        });

        let qptr = QPointer::from(&*self);

        let on_disconnected = queued_callback(move |()| {
            qptr.as_pinned().map(|this| {
                this.borrow_mut().connecting = false;
                this.borrow_mut().connected = false;
                this.borrow_mut().connectClient();
            });
        });

        let qptr = QPointer::from(&*self);

        let on_connection_lost = queued_callback(move |()| {
            qptr.as_pinned().map(|this| {
                this.borrow_mut().connecting = false;
                this.borrow_mut().connected = false;
                this.borrow_mut().connectClient();
            });
        });

        let qptr = QPointer::from(&*self);

        let on_connected = queued_callback(move |()| {
            qptr.as_pinned().map(|this| {
                this.borrow_mut().connecting = false;
                this.borrow_mut().connected = true;
            });
        });

        // let mut strm = cli.get_stream(25);

        // Create the connect options, explicitly requesting MQTT v3.x
        let mut conn_opts = mqtt::ConnectOptionsBuilder::new_v3();
        conn_opts
            .keep_alive_interval(Duration::from_secs(30))
            .clean_session(true)
            .user_name(self.username.to_string())
            .password(self.password.to_string());

        if self.enableSSL == true {
            let ssl_opts = mqtt::SslOptionsBuilder::new()
                .enable_server_cert_auth(false)
                .finalize();
            conn_opts.ssl_options(ssl_opts);
        }
        let conn_opts = conn_opts.finalize();

        cli.set_message_callback(move |_, msg_opt| {
            on_message_arrived(msg_opt);
        });
        cli.set_disconnected_callback(move |_, _, reason| {
            println!("Disconnected");
            on_disconnected(());
        });

        cli.set_connection_lost_callback(move |_| {
            println!("Connection lost.");
            on_connection_lost(());
        });
        cli.set_connected_callback(move |_| {
            println!("Connected");
            on_connected(());
        });

        let qptr = QPointer::from(&*self);

        let worker_done = queued_callback(move |()| {
            qptr.as_pinned().map(|this| {
                // begin
                println!("Worker done!");
                this.borrow_mut().connected = true;
                this.borrow_mut().connecting = false;
                // end
            });
        });

        // we're done with the client
        self.cli = Arc::new(Mutex::new(Some(cli)));

        self.connecting = true;
        let topic = self.topic.to_string();
        // TODO: can this be simplified
        let mself = self.cli.clone();

        //       let cli = cli.clone();
        std::thread::spawn(move || {
            let mut connect_success = false;
            loop {
                // Make the connection to the broker
                // clone the options so that we keep the original for use when looping
                let opts = conn_opts.clone();

                //let opts = ConnectOptions::new();
                if let Err(e) = mself.lock().unwrap().as_ref().unwrap().connect(opts).wait() {
                    println!("Can't connect: {}", e);
                    std::thread::sleep(Duration::from_secs(1));
                } else {
                    connect_success = true;
                    break;
                }
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Stopping worker thread");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }

            if connect_success {
                let topics: &[String] = &[topic];
                const QOS: &[i32] = &[1];

                println!("Subscribing to topics: {:?}", topics);
                if let Err(e) = mself
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .subscribe_many(topics, QOS)
                    .wait()
                {
                    println!("Can't subscribe: {}", e);
                }

                // Just loop on incoming messages.
                println!("Waiting for messages...");
            }
        });
    }
}
