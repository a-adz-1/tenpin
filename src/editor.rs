use crate::{
    demos, kit, library, pattern_index, seq::{TRIPLET, TRIPLET8}, Shared, Step, TenpinParams, CHANNELS,
    PATTERNS, STEPS,
};
use nih_plug::prelude::*;
use nih_plug_egui::create_egui_editor;
use nih_plug_egui::egui::{
    self, pos2, vec2, Align, Align2, Button, Color32, FontId, Frame, Layout, Margin, Pos2, Rect,
    Response, RichText, Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
};
use std::f32::consts::PI;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub const WIDTH: u32 = 1140;
pub const HEIGHT: u32 = 580;
const STRIP_W: f32 = 92.0;

const BG: Color32 = Color32::from_rgb(19, 20, 24);
const PANEL: Color32 = Color32::from_rgb(30, 32, 38);
const STRIP: Color32 = Color32::from_rgb(38, 41, 48);
const EDGE: Color32 = Color32::from_rgb(56, 60, 70);
const BTN: Color32 = Color32::from_rgb(52, 56, 66);
const TEXT: Color32 = Color32::from_rgb(214, 218, 226);
const DIM: Color32 = Color32::from_rgb(128, 134, 148);
const AMBER: Color32 = Color32::from_rgb(255, 178, 56);
const TEAL: Color32 = Color32::from_rgb(64, 204, 200);
const RED: Color32 = Color32::from_rgb(236, 86, 74);
const GREEN: Color32 = Color32::from_rgb(150, 220, 90);
/// Step colours for off, soft, medium, hard.
const VEL: [Color32; 4] = [
    Color32::from_rgb(48, 51, 60),
    Color32::from_rgb(122, 84, 34),
    Color32::from_rgb(196, 132, 42),
    AMBER,
];

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Vel,
    Roll,
    Pitch,
}

/// Editor-only state.
struct View {
    selected: usize,
    collapsed: bool,
    mode: Mode,
    /// Velocity new steps get, 1-3.
    dynamic: u8,
    /// Drag distance not yet turned into pitch steps.
    drag: f32,
}

pub fn create(params: Arc<TenpinParams>, shared: Arc<Shared>) -> Option<Box<dyn Editor>> {
    let view = View {
        selected: 0,
        collapsed: false,
        mode: Mode::Vel,
        dynamic: 3,
        drag: 0.0,
    };
    create_egui_editor(
        params.editor_state.clone(),
        view,
        |ctx, _| style(ctx),
        move |ctx, setter, view| {
            if let Some(state) = shared.pending_state.lock().unwrap().take() {
                setter.raw_context.set_state(state);
            }
            egui::CentralPanel::default()
                .frame(Frame::new().fill(BG).inner_margin(Margin::same(10)))
                .show(ctx, |ui| {
                    header(ui, setter, &params, &shared, view);
                    ui.add_space(6.0);
                    if view.collapsed {
                        grid(ui, setter, &params, &shared, view);
                    } else {
                        strips(ui, setter, &params, &shared, view);
                    }
                    ui.add_space(8.0);
                    sequencer(ui, setter, &params, &shared, view);
                });
        },
    )
}

fn style(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.panel_fill = BG;
    v.window_fill = PANEL;
    v.override_text_color = Some(TEXT);
    v.widgets.inactive.weak_bg_fill = BTN;
    v.widgets.inactive.bg_fill = BTN;
    v.widgets.hovered.weak_bg_fill = mix(BTN, TEXT, 0.15);
    v.selection.bg_fill = mix(BTN, AMBER, 0.45);
    v.selection.stroke = Stroke::new(1.0_f32, AMBER);
    ctx.set_visuals(v);
}

fn header(
    ui: &mut Ui,
    setter: &ParamSetter,
    params: &Arc<TenpinParams>,
    shared: &Arc<Shared>,
    view: &mut View,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("TENPIN").size(28.0).strong().color(AMBER));
        ui.label(RichText::new("ten channel drum machine").color(DIM));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            knob(ui, setter, &params.master, "Master", false);
            knob(ui, setter, &params.room, "Room", false);
            ui.add_space(8.0);
            let label = if view.collapsed {
                "Show knobs"
            } else {
                "Hide knobs"
            };
            if ui
                .add(Button::new(label).min_size(vec2(90.0, 26.0)))
                .clicked()
            {
                view.collapsed = !view.collapsed;
            }
            if ui
                .add(Button::new("Save").min_size(vec2(60.0, 26.0)))
                .on_hover_text("Save everything to a .tenpin file")
                .clicked()
            {
                save(setter);
            }
            if ui
                .add(Button::new("Load").min_size(vec2(60.0, 26.0)))
                .on_hover_text("Load a .tenpin file")
                .clicked()
            {
                load(shared.clone());
            }
            ui.menu_button(RichText::new("Demos").strong(), |ui| {
                for d in demos::ALL {
                    if ui.button(d.name).clicked() {
                        apply_demo(setter, params, shared, d);
                        ui.close_menu();
                    }
                }
            });
        });
    });
}

fn strips(
    ui: &mut Ui,
    setter: &ParamSetter,
    params: &Arc<TenpinParams>,
    shared: &Arc<Shared>,
    view: &mut View,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for c in 0..CHANNELS {
            strip(ui, setter, params, shared, view, c);
        }
    });
}

fn strip(
    ui: &mut Ui,
    setter: &ParamSetter,
    params: &Arc<TenpinParams>,
    shared: &Arc<Shared>,
    view: &mut View,
    c: usize,
) {
    let p = &params.ch[c];
    let selected = view.selected == c;
    Frame::new()
        .fill(STRIP)
        .stroke(Stroke::new(1.0_f32, if selected { TEAL } else { EDGE }))
        .corner_radius(6)
        .inner_margin(Margin::same(6))
        .show(ui, |ui| ui.vertical(|ui| {
            ui.set_width(STRIP_W);
            ui.spacing_mut().item_spacing = vec2(4.0, 5.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{}", c + 1)).size(18.0).strong().color(if selected { TEAL } else { TEXT }));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| led(ui, shared.levels[c].load(Ordering::Relaxed)));
            });
            sample_menu(ui, setter, params, shared, c);
            ui.horizontal(|ui| {
                toggle(ui, setter, &p.mute, "M", RED, "Mute");
                toggle(ui, setter, &p.solo, "S", GREEN, "Solo");
                toggle(ui, setter, &p.choke, "C", TEAL, "Choke: choke channels cut each other off, like a closed hat stopping an open one");
            });
            for (a, a_name, a_bi, b, b_name, b_bi) in [
                (&p.level, "Level", false, &p.pan, "Pan", true),
                (&p.pitch, "Pitch", true, &p.decay, "Decay", false),
                (&p.tone, "Tone", false, &p.start, "Start", false),
            ] {
                ui.horizontal(|ui| {
                    knob(ui, setter, a, a_name, a_bi);
                    knob(ui, setter, b, b_name, b_bi);
                });
            }
            ui.horizontal(|ui| {
                knob(ui, setter, &p.verb, "Reverb", false);
                let pad = Button::new(RichText::new("▶").size(16.0).color(AMBER)).fill(mix(BTN, AMBER, 0.12)).min_size(vec2(STRIP_W - 48.0, 44.0));
                if ui.add(pad).on_hover_text("Play this sound").clicked() {
                    shared.audition.fetch_or(1 << c, Ordering::Relaxed);
                    view.selected = c;
                }
            });
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                if pill(ui, selected, "SELECT", TEAL, vec2(STRIP_W, 20.0)).clicked() {
                    view.selected = c;
                }
            });
        }));
}

/// The all-channel step grid shown when the knobs are hidden.
fn grid(
    ui: &mut Ui,
    setter: &ParamSetter,
    params: &Arc<TenpinParams>,
    shared: &Arc<Shared>,
    view: &mut View,
) {
    let pattern = params.pattern.value() as usize;
    let steps = params.steps.value() as usize;
    let playing = shared.step.load(Ordering::Relaxed);
    Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1.0_f32, EDGE))
        .corner_radius(8)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(318.0);
            ui.spacing_mut().item_spacing = vec2(4.0, 6.0);
            for c in 0..CHANNELS {
                ui.horizontal(|ui| {
                    led(ui, shared.levels[c].load(Ordering::Relaxed));
                    let name = format!("{}  {}", c + 1, short(&sample_name(params, shared, c), 16));
                    if pill(ui, view.selected == c, &name, TEAL, vec2(160.0, 22.0)).clicked() {
                        view.selected = c;
                    }
                    toggle(ui, setter, &params.ch[c].mute, "M", RED, "Mute");
                    toggle(ui, setter, &params.ch[c].solo, "S", GREEN, "Solo");
                    ui.add_space(10.0);
                    for s in 0..STEPS {
                        if s > 0 && s % 4 == 0 {
                            ui.add_space(8.0);
                        }
                        let (rect, resp) = ui.allocate_exact_size(vec2(44.0, 22.0), Sense::click());
                        let i = pattern_index(pattern, c, s);
                        let mut step = params.patterns.read().unwrap()[i];
                        if resp.clicked() {
                            step.vel = if step.vel == view.dynamic {
                                0
                            } else {
                                view.dynamic
                            };
                        } else if resp.secondary_clicked() {
                            step = Step::default();
                        }
                        if resp.clicked() || resp.secondary_clicked() {
                            params.patterns.write().unwrap()[i] = step;
                        }
                        paint_step(ui, rect, s, step, s >= steps, s as i32 == playing, false);
                    }
                });
            }
        });
}

fn sequencer(
    ui: &mut Ui,
    setter: &ParamSetter,
    params: &Arc<TenpinParams>,
    shared: &Arc<Shared>,
    view: &mut View,
) {
    let c = view.selected;
    let pattern = params.pattern.value() as usize;
    let steps = params.steps.value() as usize;
    let playing = shared.step.load(Ordering::Relaxed);
    Frame::new().fill(PANEL).stroke(Stroke::new(1.0_f32, EDGE)).corner_radius(8).inner_margin(Margin::same(10)).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            let run = params.run.value();
            let label = RichText::new(if run { "STOP" } else { "RUN" }).size(18.0).strong().color(if run { Color32::BLACK } else { TEXT });
            let button = Button::new(label).fill(if run { AMBER } else { BTN }).min_size(vec2(76.0, 50.0));
            if ui.add(button).on_hover_text("Play without the host. In a DAW, its own play button works too.").clicked() {
                set(setter, &params.run, !run);
            }
            knob(ui, setter, &params.bpm, "BPM", false);
            knob(ui, setter, &params.shuffle, "Shuffle", false);
            knob(ui, setter, &params.steps, "Steps", false);
            ui.separator();
            section(ui, "PATTERN", |ui| {
                for p in 0..PATTERNS {
                    let name = ((b'A' + p as u8) as char).to_string();
                    if pill(ui, pattern == p, &name, AMBER, vec2(30.0, 26.0)).clicked() {
                        set(setter, &params.pattern, p as i32);
                    }
                }
            });
            ui.separator();
            section(ui, "EDIT", |ui| {
                for (mode, name, tip) in [
                    (Mode::Vel, "VEL", "Click steps on and off"),
                    (Mode::Roll, "ROLL", "Click a step for 1-4 hits inside it, then T16 and T8 for 16th and 8th triplets: hat and snare rolls"),
                    (Mode::Pitch, "PITCH", "Drag a step up or down to retune that hit"),
                ] {
                    if pill(ui, view.mode == mode, name, TEAL, vec2(52.0, 26.0)).on_hover_text(tip).clicked() {
                        view.mode = mode;
                    }
                }
            });
            ui.separator();
            section(ui, "DYNAMIC", |ui| {
                for (v, name) in [(1, "SOFT"), (2, "MED"), (3, "HARD")] {
                    if pill(ui, view.dynamic == v, name, VEL[v as usize], vec2(46.0, 26.0)).clicked() {
                        view.dynamic = v;
                    }
                }
            });
            ui.separator();
            section(ui, "CHANNEL", |ui| {
                ui.label(RichText::new(format!("{}  {}", c + 1, short(&sample_name(params, shared, c), 18))).size(16.0).color(TEAL));
                if ui.add(Button::new("Clear").min_size(vec2(50.0, 26.0))).on_hover_text("Clear this channel in this pattern").clicked() {
                    let mut pat = params.patterns.write().unwrap();
                    (0..STEPS).for_each(|s| pat[pattern_index(pattern, c, s)] = Step::default());
                }
            });
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for s in 0..STEPS {
                if s > 0 && s % 4 == 0 {
                    ui.add_space(12.0);
                }
                let (rect, resp) = ui.allocate_exact_size(vec2(58.0, 74.0), Sense::click_and_drag());
                let i = pattern_index(pattern, c, s);
                let old = params.patterns.read().unwrap()[i];
                let mut step = old;
                match view.mode {
                    Mode::Vel if resp.clicked() => step.vel = if step.vel == view.dynamic { 0 } else { view.dynamic },
                    Mode::Roll if resp.clicked() => {
                        step.roll = step.roll.clamp(1, TRIPLET8) % TRIPLET8 + 1;
                        step.vel = step.vel.max(view.dynamic.min(if step.vel == 0 { 3 } else { step.vel }));
                    }
                    Mode::Pitch if resp.dragged() => {
                        view.drag -= resp.drag_delta().y;
                        let semis = (view.drag / 8.0).trunc();
                        view.drag -= semis * 8.0;
                        step.pitch = (step.pitch as f32 + semis).clamp(-12.0, 12.0) as i8;
                    }
                    _ => {}
                }
                if resp.secondary_clicked() {
                    match view.mode {
                        Mode::Vel => step = Step::default(),
                        Mode::Roll => step.roll = 1,
                        Mode::Pitch => step.pitch = 0,
                    }
                }
                if step != old {
                    params.patterns.write().unwrap()[i] = step;
                }
                let led_at = pos2(rect.center().x, rect.top() + 5.0);
                ui.painter().circle_filled(led_at, 4.0, if s as i32 == playing { AMBER } else { VEL[0] });
                let pad = Rect::from_min_max(rect.min + vec2(0.0, 14.0), rect.max);
                paint_step(ui, pad, s, step, s >= steps, false, true);
                let big = match view.mode {
                    Mode::Roll if step.roll == TRIPLET => "T16".into(),
                    Mode::Roll if step.roll == TRIPLET8 => "T8".into(),
                    Mode::Roll if step.hits() > 1 => format!("×{}", step.hits()),
                    Mode::Pitch if step.pitch != 0 => format!("{:+}", step.pitch),
                    _ => String::new(),
                };
                ui.painter().text(pad.center(), Align2::CENTER_CENTER, big, FontId::proportional(18.0), Color32::BLACK);
            }
        });
        ui.label(
            RichText::new("Right-click a step to reset it. Drag knobs, shift for fine, double-click for default. MIDI C1-A1 plays channels 1-10.")
                .small()
                .color(DIM),
        );
    });
}

fn paint_step(ui: &Ui, rect: Rect, s: usize, step: Step, beyond: bool, playing: bool, big: bool) {
    let painter = ui.painter();
    let mut fill = VEL[step.vel.min(3) as usize];
    if step.vel == 0 && (s / 4) % 2 == 1 {
        fill = mix(fill, TEXT, 0.06);
    }
    if beyond {
        fill = fill.gamma_multiply(0.35);
    }
    painter.rect_filled(rect.shrink(1.0), 5, fill);
    if playing {
        painter.rect_stroke(
            rect,
            5,
            Stroke::new(2.0f32, Color32::WHITE),
            StrokeKind::Inside,
        );
    }
    let ink = if step.vel > 1 {
        Color32::from_black_alpha(170)
    } else {
        DIM
    };
    if big {
        painter.text(
            rect.left_bottom() + vec2(6.0, -5.0),
            Align2::LEFT_BOTTOM,
            format!("{}", s + 1),
            FontId::proportional(11.0),
            ink,
        );
    }
    if step.vel > 0 && step.pitch != 0 {
        painter.text(
            rect.right_top() + vec2(-5.0, 4.0),
            Align2::RIGHT_TOP,
            format!("{:+}", step.pitch),
            FontId::proportional(10.0),
            ink,
        );
    }
    if step.vel > 0 && step.roll >= TRIPLET {
        painter.text(
            rect.right_bottom() + vec2(-5.0, -2.0),
            Align2::RIGHT_BOTTOM,
            if step.roll == TRIPLET8 { "T8" } else { "T16" },
            FontId::proportional(10.0),
            ink,
        );
    } else if step.vel > 0 && step.hits() > 1 {
        let y = rect.bottom() - 6.0;
        for h in 0..step.hits() {
            let x = rect.right() - 7.0 - h as f32 * 6.0;
            painter.circle_filled(pos2(x, y), 2.0, ink);
        }
    }
}

/// A label above a row of controls.
fn section(ui: &mut Ui, title: &str, add: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).size(10.0).color(DIM));
        ui.horizontal(add);
    });
}

fn sample_menu(
    ui: &mut Ui,
    setter: &ParamSetter,
    params: &Arc<TenpinParams>,
    shared: &Arc<Shared>,
    c: usize,
) {
    let p = &params.ch[c];
    let name = sample_name(params, shared, c);
    let button =
        Button::new(RichText::new(short(&name, 13)).size(11.0)).min_size(vec2(STRIP_W, 22.0));
    let current = p.sample.value() as usize;
    let has_user = shared.user.read().unwrap()[c].is_some();
    egui::menu::menu_custom_button(ui, button, |ui| {
        for cat in library::CATEGORIES {
            let items: Vec<usize> = (0..library::len())
                .filter(|&i| library::category(library::name(i)) == *cat)
                .collect();
            if items.is_empty() {
                continue;
            }
            ui.menu_button(cat.replace('_', " "), |ui| {
                for i in items {
                    if ui
                        .selectable_label(i == current && !has_user, library::label(i))
                        .clicked()
                    {
                        set(setter, &p.sample, i as i32);
                        shared.user.write().unwrap()[c] = None;
                        params.paths.write().unwrap()[c].clear();
                        shared.audition.fetch_or(1 << c, Ordering::Relaxed);
                        ui.close_menu();
                    }
                }
            });
        }
        ui.separator();
        if ui.button("Load WAV…").clicked() {
            pick_sample(c, params.clone(), shared.clone());
            ui.close_menu();
        }
    })
    .response
    .on_hover_text(name);
}

fn sample_name(params: &TenpinParams, shared: &Shared, c: usize) -> String {
    match &shared.user.read().unwrap()[c] {
        Some(s) => s.name.clone(),
        None => library::name(params.ch[c].sample.value().max(0) as usize).replace('_', " "),
    }
}

fn short(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.into()
    } else {
        s.chars().take(n - 1).chain(['…']).collect()
    }
}

/// A rotary knob: drag up/down (shift for fine), scroll, double-click to reset.
fn knob<P: Param>(ui: &mut Ui, setter: &ParamSetter, param: &P, label: &str, bipolar: bool) {
    let (rect, resp) = ui.allocate_exact_size(vec2(44.0, 50.0), Sense::click_and_drag());
    let norm = param.unmodulated_normalized_value();
    let to = |v: f32| setter.set_parameter_normalized(param, v.clamp(0.0, 1.0));
    if resp.drag_started() {
        setter.begin_set_parameter(param);
    }
    if resp.dragged() {
        let fine = ui.input(|i| i.modifiers.shift);
        to(norm - resp.drag_delta().y * if fine { 0.001 } else { 0.005 });
    }
    if resp.drag_stopped() {
        setter.end_set_parameter(param);
    }
    if resp.double_clicked() {
        set(setter, param, param.default_plain_value());
    }
    let scroll = if resp.hovered() {
        ui.input(|i| i.raw_scroll_delta.y)
    } else {
        0.0
    };
    if scroll != 0.0 {
        setter.begin_set_parameter(param);
        to(norm + scroll.signum() * 0.02);
        setter.end_set_parameter(param);
    }

    let painter = ui.painter();
    let centre = pos2(rect.center().x, rect.top() + 18.0);
    let r = 14.0;
    let at = |t: f32| {
        let a = (0.75 + 1.5 * t) * PI;
        centre + vec2(a.cos(), a.sin()) * r
    };
    let arc = |from: f32, to: f32| -> Vec<Pos2> {
        (0..=24)
            .map(|i| at(from + (to - from) * i as f32 / 24.0))
            .collect()
    };
    painter.circle_filled(centre, r + 3.0, BG);
    painter.add(Shape::line(arc(0.0, 1.0), Stroke::new(3.0_f32, EDGE)));
    let from = if bipolar { 0.5 } else { 0.0 };
    if (norm - from).abs() > 0.002 {
        painter.add(Shape::line(arc(from, norm), Stroke::new(3.0_f32, AMBER)));
    }
    painter.circle_filled(centre, r - 4.0, mix(STRIP, TEXT, 0.12));
    let a = (0.75 + 1.5 * norm) * PI;
    let dir = vec2(a.cos(), a.sin());
    painter.line_segment(
        [centre + dir * 2.0, centre + dir * (r - 5.0)],
        Stroke::new(2.0_f32, TEXT),
    );
    let active = resp.hovered() || resp.dragged();
    let text = if active {
        param.normalized_value_to_string(norm, true)
    } else {
        label.to_string()
    };
    painter.text(
        pos2(rect.center().x, rect.bottom() - 6.0),
        Align2::CENTER_CENTER,
        text,
        FontId::proportional(10.5),
        if active { AMBER } else { DIM },
    );
}

fn led(ui: &mut Ui, level: f32) {
    let (rect, _) = ui.allocate_exact_size(vec2(10.0, 10.0), Sense::hover());
    ui.painter().circle_filled(
        rect.center(),
        4.0,
        mix(VEL[0], AMBER, (level * 3.0).clamp(0.0, 1.0)),
    );
}

fn pill(ui: &mut Ui, on: bool, text: &str, color: Color32, size: Vec2) -> Response {
    let label = RichText::new(text)
        .strong()
        .color(if on { Color32::BLACK } else { TEXT });
    ui.add(
        Button::new(label)
            .fill(if on { color } else { BTN })
            .min_size(size),
    )
}

fn toggle(
    ui: &mut Ui,
    setter: &ParamSetter,
    param: &BoolParam,
    label: &str,
    color: Color32,
    tip: &str,
) {
    if pill(ui, param.value(), label, color, vec2(26.0, 20.0))
        .on_hover_text(tip)
        .clicked()
    {
        set(setter, param, !param.value());
    }
}

fn set<P: Param>(setter: &ParamSetter, param: &P, value: P::Plain) {
    setter.begin_set_parameter(param);
    setter.set_parameter(param, value);
    setter.end_set_parameter(param);
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    Color32::from_rgb(f(a.r(), b.r()), f(a.g(), b.g()), f(a.b(), b.b()))
}

fn apply_demo(setter: &ParamSetter, params: &TenpinParams, shared: &Shared, d: &demos::Demo) {
    set(setter, &params.bpm, d.bpm);
    set(setter, &params.shuffle, d.shuffle);
    set(setter, &params.steps, STEPS as i32);
    set(setter, &params.pattern, 0);
    for (c, k) in d.kit.iter().enumerate() {
        let p = &params.ch[c];
        set(
            setter,
            &p.sample,
            library::find(k.sample).unwrap_or(c) as i32,
        );
        set(setter, &p.level, util::db_to_gain(k.level));
        set(setter, &p.pitch, k.pitch);
        set(setter, &p.decay, k.decay);
        set(setter, &p.tone, k.tone);
        set(setter, &p.pan, k.pan);
        set(setter, &p.start, 0.0);
        set(setter, &p.mute, false);
        set(setter, &p.solo, false);
        set(setter, &p.choke, k.choke);
        set(setter, &p.verb, k.verb);
    }
    *params.patterns.write().unwrap() = d.steps();
    params
        .paths
        .write()
        .unwrap()
        .iter_mut()
        .for_each(String::clear);
    shared
        .user
        .write()
        .unwrap()
        .iter_mut()
        .for_each(|u| *u = None);
}

/// Save the whole plugin state, the same thing a DAW stores in its project.
fn save(setter: &ParamSetter) {
    let state = setter.raw_context.get_state();
    let Ok(json) = serde_json::to_string_pretty(&state) else {
        return;
    };
    std::thread::spawn(move || {
        let dialog = rfd::FileDialog::new()
            .add_filter("Tenpin", &["tenpin"])
            .set_file_name("jam.tenpin");
        if let Some(path) = dialog.save_file() {
            if let Err(e) = std::fs::write(&path, json) {
                nih_error!("could not save {}: {e}", path.display());
            }
        }
    });
}

/// Read a .tenpin file on its own thread; the editor applies it on its next frame.
fn load(shared: Arc<Shared>) {
    std::thread::spawn(move || {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Tenpin", &["tenpin"])
            .pick_file()
        else {
            return;
        };
        match std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|s| serde_json::from_str(&s).map_err(|e| e.to_string()))
        {
            Ok(state) => *shared.pending_state.lock().unwrap() = Some(state),
            Err(e) => nih_error!("could not load {}: {e}", path.display()),
        }
    });
}

/// Open the file picker on its own thread so the host's UI never blocks on it.
fn pick_sample(ch: usize, params: Arc<TenpinParams>, shared: Arc<Shared>) {
    std::thread::spawn(move || {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("WAV", &["wav", "WAV"])
            .pick_file()
        else {
            return;
        };
        match kit::load_wav(&path) {
            Ok(sample) => {
                params.paths.write().unwrap()[ch] = path.to_string_lossy().into_owned();
                shared.user.write().unwrap()[ch] = Some(Arc::new(sample));
                shared.audition.fetch_or(1 << ch, Ordering::Relaxed);
            }
            Err(e) => nih_error!("could not load {}: {e}", path.display()),
        }
    });
}
