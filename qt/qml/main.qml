import QtQuick
import QtQuick.Controls
import QtQuick.Window

import com.syler.payrollpayments 1.0

Window {
    height: 480
    title: qsTr("Hello World2")
    visible: true
    width: 640
    

    MyObject {
        id: myObject
        number: 1
        string: qsTr("My String with my number: %1").arg(myObject.number)
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(myObject.number)
        }

        Label {
            text: qsTr("String: %1").arg(myObject.string)
        }

        Button {
            text: qsTr("Increment Number")

            onClicked: myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")

            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }
    }
}
// ANCHOR_END: book_main_qml
