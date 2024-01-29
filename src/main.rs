use slint::ComponentHandle;

fn main() {
    let main_window = MainWindow::new().unwrap();

    main_window.run().unwrap();
}

slint::slint!{
    import { StandardButton , Button} from "std-widgets.slint";
export component MainWindow inherits Window {
        icon: @image-url("icon/icon.png");
        title: "Calculator";
        min-width: 270px;
        min-height: 480px;
        preferred-width: 324px;
        preferred-height: 480px;

        VerticalLayout {
            HorizontalLayout {
                Rectangle { background: orange;}
            }
            HorizontalLayout {
                Rectangle {
                    background: red;
                }
                Rectangle {
                    background: green;
                }
                Rectangle {
                    background: blue;
                }
                Rectangle {
                    background: yellow;
                }
            }
            HorizontalLayout {
                Rectangle {
                    background: green;
                }
                Rectangle {
                    background: blue;
                }
                Rectangle {
                    background: yellow;
                }
                Rectangle {
                    background: red;
                }
            }
            HorizontalLayout {
                Rectangle {
                    background: blue;
                }
                Rectangle {
                    background: yellow;
                }
                Rectangle {
                    background: red;
                }
                Rectangle {
                    background: green;
                }
            }
            HorizontalLayout {
                Rectangle {
                    background: yellow;
                }
                Rectangle {
                    background: red;
                }
                Rectangle {
                    background: green;
                }
                Rectangle {
                    background: blue;
                }
            }
        }
    }
}
