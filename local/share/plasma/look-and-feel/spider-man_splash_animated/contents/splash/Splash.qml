// Preview: "../previews/preview_fs-jazz-animated.png"

// SPDX‑License‑Identifier: GPL‑3.0‑or‑later
/*
    A mid-to-late 90s-inspired KDE Plasma splash screen featuring a 'Flying Toaster' and a vibrant 'Fruit Stripe Gum' color palette.
    Copyright (C) 2025 Jim Sines <raystanza@raystanza.uk>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
import QtQuick 2.15
import QtQuick.Window 2.15

Item {
    id: root
    width: Screen.width
    height: Screen.height

    // System injected properties
    property int stage
    property int progress

    // Theme colors
    property var theme: ({
        primary: "#D80000",
        secondary: "#000000",
        text: "#FEFEFE",
        border: "#D80000",
        accent: "#0047AB",
        win95teal: "#008080"
    })

    // Font Loader
    FontLoader {
        id: pixelFont
        source: "fonts/Pixel_Emulator.otf"
    }

    // Background Image
    Rectangle {
        id: background
        anchors.fill: parent
        color: "#000000"
    }

    // Centered Spider-Man Head Image
    Image {
        id: spider_head
        width: 256
        height: 256
        fillMode: Image.PreserveAspectFit
        source: "images/Face_Spider_man.png"
        anchors.centerIn: parent
        smooth: true
    }

    // Floating Animation
    PropertyAnimation {
        target: spider_head
        property: "y"
        from: spider_head.y
        to: spider_head.y - 10
        duration: 1000
        running: true
        loops: Animation.Infinite
        easing.type: Easing.InOutSine
    }

    // Breathing Animation
    SequentialAnimation {
        running: true
        loops: Animation.Infinite
        PropertyAnimation {
            target: spider_head
            property: "scale"
            from: 1.0
            to: 1.05
            duration: 500
            easing.type: Easing.InOutQuad
        }
        PropertyAnimation {
            target: spider_head
            property: "scale"
            from: 1.05
            to: 1.0
            duration: 500
            easing.type: Easing.InOutQuad
        }
    }

    // "Loading" Text with Background
    Rectangle {
        id: loadingContainer
        width: loadingText.paintedWidth + 32
        height: loadingText.paintedHeight + 16
        color: theme.secondary
        border.color: theme.border
        border.width: 3
        radius: 10
        anchors.top: spider_head.bottom
        anchors.topMargin: 40
        anchors.horizontalCenter: parent.horizontalCenter

        Text {
            id: loadingText
            text: "Loading..."
            anchors.centerIn: parent
            font.pixelSize: 32
            font.family: pixelFont.name
            color: theme.text
            font.bold: true
            font.letterSpacing: 1.2
            smooth: true
        }
    }

    // Progress Bar
    Rectangle {
        id: progressBarBackground
        width: parent.width * 0.4
        height: 16
        radius: 8
        border {
            width: 2
            color: theme.border
        }
        color: theme.secondary
        anchors.top: loadingContainer.bottom
        anchors.topMargin: 20
        anchors.horizontalCenter: parent.horizontalCenter

        Rectangle {
            id: progressBarFill
            width: progressBarBackground.width * Math.max(0, Math.min(1, root.progress))
            height: parent.height
            radius: 8
            color: theme.primary

            Behavior on width {
                NumberAnimation {
                    duration: 250
                    easing.type: Easing.InOutQuad
                }
            }
        }

        Text {
            text: Math.round(root.progress * 100) + "%"
            anchors.top: progressBarBackground.bottom
            anchors.topMargin: 8
            anchors.horizontalCenter: parent.horizontalCenter
            font.pixelSize: 18
            font.family: pixelFont.name
            color: theme.text
        }

    }

    // Plasma-specific stage handling
    onStageChanged: {
    switch (stage) {
        case 1:
            loadingText.text = "Inicializando...";
            root.progress = 0.2;
            break;
        case 2:
            loadingText.text = "Cargando Spider-Servicios...";
            root.progress = 0.4;
            break;
        case 3:
            loadingText.text = "Iniciando Spider-Desktop...";
            root.progress = 0.6;
            break;
        case 4:
            loadingText.text = "Preparando...";
            root.progress = 0.8;
            break;
        case 5:
            loadingText.text = "Listo";
            root.progress = 1.0;
            break;
        }
    }
}
