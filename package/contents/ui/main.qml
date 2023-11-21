// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
// Author: Simon Brummer (simon.brummer@posteo.de)
// Description: Widget used to read/write data from the Qml Extension
//              that is implemented in Rust.

import QtQuick 2.1
import QtQuick.Layouts 1.1
import org.kde.plasma.components 2.0 as PlasmaComponents
import org.kde.plasma.core 2.0 as PlasmaCore
import org.kde.plasma.plasmoid 2.0
import org.kde.plasma.private.mqtt 1.0

Rectangle {
    // implicitWidth: childrenRect.implicitWidth + 20
    // implicitHeight: childrenRect.implicitHeight + 20

    id: rootelement

    property MQTTConnection newConnection
    property string mqttValue: "0"
    property string lastUpdate: formatDate(new Date())

    Plasmoid.onUserConfiguringChanged: {
        if (!plasmoid.userConfiguring) {
            // TODO: check if url and or username password changed.
            // Otherwise no need to recreate connection
            // Just disconnect and reconnect
             mqttconnection.disconnectClient();
             mqttconnection.connectClient()
         }
     }

    function formatDate(date) {
        let hours = date.getHours().toString().padStart(2, '0');
        let minutes = date.getMinutes().toString().padStart(2, '0');
        let seconds = date.getSeconds().toString().padStart(2, '0');
        return `${hours}:${minutes}:${seconds}`;
    }

    Component.onCompleted: {
        //MQTTPlugin.
        //Backend.number = plasmoid.configuration.counterStartValue
        //   newConnection = MQTTPlugin.getConnection("jetpack","energy/electricity/current");
        //    newConnection.onMessageArrived.connect( function() {
        //          //var script = plasmoid.configuration.script.replace("[%s]",message)
        //          //mqttValue = eval(script);
        //          mqttValue = message;
        //          lastUpdate = formatDate(new Date());
        //    });
        mqttconnection.connectClient();
    }

    MQTTConnection {
        id: mqttconnection

        name: plasmoid.configuration.name
        topic: plasmoid.configuration.topic
        address: plasmoid.configuration.address
        port: plasmoid.configuration.port
        enableSSL: plasmoid.configuration.enablessl
        trustSelfSignedCert: plasmoid.configuration.trustselfsigned
        username: plasmoid.configuration.username
        password: plasmoid.configuration.password
        onMessageArrived: {
            //var script = plasmoid.configuration.script.replace("[%s]",message)
            //mqttValue = eval(script);
            mqttValue = message;
            lastUpdate = formatDate(new Date());
        }
    }

    anchors.fill: parent
    border.color: "gray"


    ColumnLayout {
        anchors.fill: parent

        RowLayout {
            //Rectangle {
            //anchors.centerIn: parent
            //width: 300
            //height: 200
            //border.width: 2
            //Charts.LineChart {
            //anchors.fill: parent
            //colorSource: Charts.ArraySource { array: ["red", "green", "blue"] }
            //nameSource: Charts.ArraySource { array: ["First", "Second", "Third"] }
            //valueSources: [
            //Charts.ArraySource { array: [1, 2, 2, 1] },
            //Charts.ArraySource { array: [2, 5, 2, 5] },
            //Charts.ArraySource { array: [5, 4, 3, 4] }
            //]
            //}
            //}

            Layout.fillWidth: true
            Layout.fillHeight: true

            // Image {
            //     fillMode: Image.PreserveAspectFit
            //     source: plasmoid.configuration.icon
            // }
            ColumnLayout {
                PlasmaComponents.Label {
                    id: lblTitle

                    Layout.fillWidth: true
                    horizontalAlignment: Text.AlignHCenter
                    font.pointSize: 12
                    text: plasmoid.configuration.title
                    fontSizeMode: Text.Fit
                }

                PlasmaComponents.Label {
                    id: lblMqttValue

                    Layout.fillWidth: true
                    horizontalAlignment: Text.AlignHCenter
                    color: plasmoid.configuration.valuecolor
                    font.pointSize: 14
                    font.bold: true
                    text: rootelement.mqttValue
                    fontSizeMode: Text.Fit
                }

            }

        }

        PlasmaComponents.Label {
            id: lblLastUpdate

            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            color: "gray"
            font.pointSize: 8
            font.bold: false
            text: rootelement.lastUpdate
            fontSizeMode: Text.Fit
        }

    }

}
