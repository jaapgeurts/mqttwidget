/*
    SPDX-FileCopyrightText:  2020 Jaap Geurts <undisclosed>
    SPDX-License-Identifier: LGPL-2.1-or-later
*/

import QtQuick 2.0
import QtQuick.Controls 2.5 as Controls
import QtQuick.Layouts 1.1
import org.kde.kirigami 2.4 as Kirigami
// import org.kde.plasma.plasmoid 2.0
// import org.kde.plasma.core 2.0 as PlasmaCore
// RowLayout
import org.kde.kquickcontrols 2.0 as KQuickControls
import org.kde.kquickcontrolsaddons 2.0 as KQuickAddons

Kirigami.FormLayout {
    property alias cfg_name: txtName.text
    property alias cfg_title: txtTitle.text
    property alias cfg_valuecolor: btnColorValue.color
    property alias cfg_topic: txtTopic.text
    property alias cfg_address: txtAddress.text
    property alias cfg_port: txtPort.text
    property alias cfg_enablessl: chkEnableSSL.checked
    property alias cfg_trustselfsigned: chkTrustSelfSigned.checked
    property alias cfg_username: txtUsername.text
    property alias cfg_password: txtPassword.text

    Layout.fillWidth: true

    RowLayout {
        Kirigami.FormData.label: i18n("Name:")

        Controls.TextField {
            id: txtName

            Layout.fillWidth: true
            placeholderText: i18n("Settings name")
            text: serverSettings.name
            onTextChanged: settingValueChanged()
        }

    }

    RowLayout {
        Kirigami.FormData.label: i18n("Title:")

        Controls.TextField {
            id: txtTitle

            Layout.fillWidth: true
            placeholderText: i18n("Tile title")
            text: serverSettings.title
            onTextChanged: settingValueChanged()
        }

    }

    RowLayout {
        Kirigami.FormData.label: i18n("Value color:")

        KQuickControls.ColorButton {
            id: btnColorValue

            showAlphaChannel: false
        }

    }

    RowLayout {
        Kirigami.FormData.label: i18n("Topic:")

        Controls.TextField {
            id: txtTopic

            Layout.fillWidth: true
            placeholderText: i18n("Topic")
            text: serverSettings.topic
            onTextChanged: settingValueChanged()
        }

    }

    Item {
        Kirigami.FormData.isSection: true
    }

    RowLayout {
        Kirigami.FormData.label: i18n("Address:")

        Controls.TextField {
            id: txtAddress

            Layout.fillWidth: true
            placeholderText: i18n("<server address>")
            onTextChanged: settingValueChanged()
        }

    }

    RowLayout {
        Kirigami.FormData.label: i18n("Port:")

        Controls.TextField {
            id: txtPort

            placeholderText: i18n("<server port>")
            onTextChanged: settingValueChanged()
        }

    }

    Item {
        Kirigami.FormData.isSection: true
    }

    RowLayout {
        Kirigami.FormData.label: i18n("Secure connection:")

        Controls.CheckBox {
            id: chkEnableSSL

            text: i18n("Enable SSL")
            onCheckedChanged: settingValueChanged()
            onClicked: {
                if (checked)
                    txtPort.text = "8883";
                else
                    txtPort.text = "1883";
            }
        }

    }

    Controls.CheckBox {
        id: chkTrustSelfSigned

        text: i18n("Trust self signed certs")
        onCheckedChanged: settingValueChanged()
    }

    Item {
        Kirigami.FormData.isSection: true
    }

    RowLayout {
        Kirigami.FormData.label: i18n("Username:")

        Controls.TextField {
            id: txtUsername

            Layout.fillWidth: true
            onTextChanged: settingValueChanged()
        }

    }

    RowLayout {
        Kirigami.FormData.label: i18n("Password:")

        Controls.TextField {
            id: txtPassword

            echoMode: TextInput.Password
            Layout.fillWidth: true
            onTextChanged: settingValueChanged()
        }

    }

}
