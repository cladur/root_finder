use eframe::{
    egui::{self, plot::Points},
    epi,
};
use egui::plot::{Line, Plot, VLine, Value, Values};
use methods::{find_root, ComputeError, ComputeOutput, StopCriteria};

mod methods;

struct AppState {
    range_left: f64,
    range_right: f64,

    stop_criteria: StopCriteria,
    compute_result: Option<Result<ComputeOutput, ComputeError>>,

    equal_aspect_ratio: bool,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            range_left: 0.0,
            range_right: 1.0,
            stop_criteria: StopCriteria::Iterations(10),
            compute_result: None,
            equal_aspect_ratio: false,
        }
    }

    fn compute(&mut self) {
        let result = find_root(self.range_left, self.range_right, &self.stop_criteria);
        self.compute_result = Some(result);
    }
}

impl epi::App for AppState {
    fn name(&self) -> &str {
        "Root Finder"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Range selection
                ui.heading("Range");
                ui.horizontal(|ui| {
                    ui.label("Left");
                    ui.add(egui::DragValue::new(&mut self.range_left));
                    ui.label("Right");
                    ui.add(egui::DragValue::new(&mut self.range_right));
                });

                ui.add_space(20.0);

                // Stop criteria
                ui.heading("Stop Criteria");
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut self.stop_criteria,
                        StopCriteria::Iterations(100),
                        "Iterations",
                    );
                    ui.radio_value(
                        &mut self.stop_criteria,
                        StopCriteria::Epsilon(0.001),
                        "Epsilon",
                    );
                });

                // Stop criteria parameters
                match self.stop_criteria {
                    StopCriteria::Iterations(ref mut iterations) => {
                        ui.label("Number of iterations");
                        ui.add(egui::Slider::new(iterations, 1..=100));
                    }
                    StopCriteria::Epsilon(ref mut epsilon) => {
                        ui.label("Value of epsilon");
                        ui.add(egui::Slider::new(epsilon, 1e-100..=0.1));
                    }
                }

                ui.add_space(20.0);

                // Run button
                if ui.button("Run").clicked() {
                    self.compute();
                }

                ui.add_space(20.0);

                ui.checkbox(&mut self.equal_aspect_ratio, "Equal aspect ratio");

                ui.add_space(20.0);

                // Light / Dark
                ui.horizontal(|ui| {
                    if ui.button("Light").clicked() {
                        ctx.set_visuals(egui::style::Visuals::light());
                    }
                    if ui.button("Dark").clicked() {
                        ctx.set_visuals(egui::style::Visuals::dark());
                    }
                });

                ui.add_space(20.0);

                match &self.compute_result {
                    Some(Ok(result)) => {
                        ui.heading("Bisection method");
                        ui.label(format!("X: {}", result.bisection.x));
                        ui.label(format!("Y: {:.3e}", result.bisection.y));
                        ui.label(format!("Iterations: {}", result.bisection.iterations));
                        ui.heading("Newton method");
                        ui.label(format!("X: {}", result.newton.x));
                        ui.label(format!("Y: {:.3e}", result.newton.y));
                        ui.label(format!("Iterations: {}", result.newton.iterations));
                    }
                    Some(Err(error)) => match error {
                        ComputeError::SameSign => {
                            ui.colored_label(
                                egui::Color32::RED,
                                "Function has same sign on both sides of the range.",
                            );
                        }
                    },
                    None => {}
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.compute_result {
            Some(Ok(compute_result)) => {
                let values = Values::from_values(compute_result.values.clone());
                let line = Line::new(values);

                let bisec = Points::new(Values::from_values(vec![Value::new(
                    compute_result.bisection.x,
                    compute_result.bisection.y,
                )]))
                .radius(8.0)
                .name("Bisection");

                let newton = Points::new(Values::from_values(vec![Value::new(
                    compute_result.newton.x,
                    compute_result.newton.y,
                )]))
                .radius(8.0)
                .shape(egui::plot::MarkerShape::Square)
                .name("Newton");

                let vline_left = VLine::new(self.range_left);
                let vline_right = VLine::new(self.range_right);

                let mut plot = Plot::new("my_plot")
                    .show_x(false)
                    .show_y(false)
                    .legend(egui::widgets::plot::Legend::default());

                if self.equal_aspect_ratio {
                    plot = plot.data_aspect(1.0);
                }

                plot.show(ui, |plot_ui| {
                    plot_ui.line(line);
                    plot_ui.vline(vline_left);
                    plot_ui.vline(vline_right);
                    plot_ui.points(bisec);
                    plot_ui.points(newton);
                });
            }
            None | Some(Err(_)) => {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.heading("No data");
                    },
                );
            }
        });
    }
}

fn main() {
    let app = AppState::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
