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
            spacing: 8px;
            padding: 8px;
            HorizontalLayout {
                Rectangle {
                    height: output.height;
                    background: #FF8D8B;
                    border-radius: 5px;
                    output := Text {
                        x: parent.x + 8px;
                        width: parent.width - 16px;
                        min-height: 48px;
                        wrap: word-wrap;
                        horizontal-alignment: left;
                        vertical-alignment: center;
                        font-size: 24px;
                        text: "";
                    }
                }
            }
            HorizontalLayout {
                spacing: 8px;
                Button {
                    text: "<";
                    // clicked => {
                    //     output.text = "";
                    // }
                }
                Button {
                    text: "C";
                    clicked => {
                        output.text = "";
                    }
                }
            }
            HorizontalLayout {
                spacing: 8px;
                Button {
                    text: "(";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: ")";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "^";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "/";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
            }
            HorizontalLayout {
                spacing: 8px;
                Button {
                    text: "7";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "8";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "9";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "*";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
            }
            HorizontalLayout {
                spacing: 8px;
                Button {
                    text: "4";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "5";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "6";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "-";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
            }
            HorizontalLayout {
                spacing: 8px;
                Button {
                    text: "1";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "2";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "3";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "+";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
            }
            HorizontalLayout {
                spacing: 8px;
                Button {
                    text: "0";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: ".";
                    clicked => {
                        output.text = output.text + self.text;
                    }
                }
                Button {
                    text: "(-)";
                    clicked => {
                        output.text = output.text + "-";
                    }
                }
                Button {
                    text: "=";
                    // clicked => {
                    //     output.text = output.text + self.text;
                    // }
                }
            }
        }
    }
}
