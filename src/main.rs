use slint::ComponentHandle;

fn main() {
    let main_window = MainWindow::new().unwrap();

    main_window.run().unwrap();
}

slint::slint!{
    export component MainWindow inherits Window {
        icon: @image-url("icon/icon.png");
        title: "";
        min-width: 480px;
        min-height: 270px;
        preferred-width: 960px;
        preferred-height: 540px;

        area := TouchArea {
            x: rect2.x;
            y: rect2.y;
            width: rect2.width;
            height: rect2.height;
        }

        rect1 := Rectangle {
            x: 0px;
            y: 0px;
            width: 240px;
            height: parent.height;
            background: red;
        }

        rect2 := Rectangle {
            x: rect1.x + rect1.width;
            y: 0px;
            width: 8px;
            height: parent.height;
            background: area.pressed ? yellow: green;
        }

        rect3 := Rectangle {
            x: rect2.x + rect2.width;
            y: 0px;
            width: parent.width;
            height: parent.height;
            background: blue;
        }
    }
}
