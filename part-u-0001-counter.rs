slint::slint! {
    import { Button } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Slint Counter by Mustafa";
        width: 300px;
        height: 400px;

        in-out property <int> counter: 0;

        callback request-increase();
        callback request-decrease();

        VerticalLayout {
            alignment: center;
            spacing: 20px;
            padding: 20px;

            Text {
                text: "Pure Rust & Slint Counter";
                font-size: 16px;
                horizontal-alignment: center;
            }
            Text {
                text: counter;
                font-size: 48px;
                horizontal-alignment: center;
                color: #4a90e2;
            }

            HorizontalLayout {
                spacing: 15px;
                alignment: center;

                Button {
                    text: "-";
                    clicked => {
                        root.request-decrease();
                    }
                }

                Button {
                    text: "+";
                    clicked => {
                        root.request-increase();
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_decrease(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() - 1);
    });

    let ui_handle = ui.as_weak();
    ui.on_request_increase(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run()
}
