use yew::prelude::*;
use chrono::{NaiveDate, NaiveTime, Weekday, Datelike, Timelike};
use chrono::prelude::*;

use crate::components::appointment::Appointment;

use crate::components::rules::{Rule, IntervalRule};

pub struct State {
    appointments: Vec<Appointment>,
}


pub struct Calendar {
    state: State,
}

impl Component for Calendar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        /*let test_appointment = Appointment::new(
            NaiveDate::from_isoywd(2020, 44, Weekday::Mon),
            None,
            None,
            "Nick: Stausaugen".to_string(),
            "Flur, Küche, Bäder".to_string(),
        );
        let test_appointment2 = Appointment::new(
            NaiveDate::from_isoywd(2020, 44, Weekday::Mon),
            Some(NaiveTime::from_hms(18, 0, 0)),
            Some(NaiveTime::from_hms(20, 0, 0)),
            "Flo: Tanzkurs".to_string(),
            "Tanzkurs in Tübingen".to_string(),
        );
        let test_appointment3 = Appointment::new(
            NaiveDate::from_isoywd(2020, 44, Weekday::Thu),
            Some(NaiveTime::from_hms(15, 0, 0)),
            None,
            "Donnerstagstermin".to_string(),
            "Ein Donnerstagstermin".to_string(),
        );*/

        let ir = IntervalRule::new(
            0,
            NaiveDate::from_isoywd(2021, 02, Weekday::Thu),
            2,
            "Putzfimmel haben".to_string(),
            "ALLES saubermachen, egal ob dreckig oder nicht".to_string(),
            None,
            None,
        );

        let now = Local::now();

        let state = State {
            appointments: ir.get_occurrences_in_week(now.year(), now.iso_week().week())
        };
        Calendar {
            state
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html{
        html! {
            <>
            <div class="row">
            <div class="col">
            <table class= { "table" } >
                <thead>
                <th> { "Montag" }</th>
                <th> { "Dienstag" }</th>
                <th> { "Mittwoch" }</th>
                <th> { "Donnerstag" }</th>
                <th> { "Freitag" }</th>
                <th> { "Samstag" }</th>
                <th> { "Sonntag" }</th>
                </thead>
                <tbody>
                    <tr>
                        <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Mon).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                        <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Tue).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                        <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Wed).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                        <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Thu).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                        <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Fri).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                       <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Sat).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                        <td>
                        <table class="table table-hover">
                            {
                                for self.state.appointments.iter().filter(|x| x.get_day().weekday() == Weekday::Sun).map(|app| self.view_entry(app))
                            }
                        </table>
                        </td>
                    </tr>

                    /*<tr>
                        <td rowspan="2" colspan="7">{ "test2" }</td>
                    </tr>*/
                </tbody>
            </table>
            </div>
            </div>

            </>
        }
    }
}

impl Calendar {
    /// returns the hour and minute of a NaiveTime in a string formatted as hh:mm
    fn format_time_hour(time: &NaiveTime) -> String {
        fn format_zeroes(t: u32) -> String {
            if t < 10 {
                format!("0{}", t.to_string())
            } else {
                t.to_string()
            }
        }
        format!("{}:{}", format_zeroes(time.hour()), format_zeroes(time.minute()))
    }

    fn view_entry(&self, app: &Appointment) -> Html {
        let start = match &app.get_start() {
            Some(start) => Calendar::format_time_hour(start),
            None => "Heute".to_string()
        };
        let end = match &app.get_end() {
            Some(end) => format!("bis {}", Calendar::format_time_hour(end)),
            None => "".to_string()
        };
        html! {
            <tr>
                <strong>
                    { start } { " " } { end }<br/>
                    { &app.get_header() }  <br/>
                </strong>
                { &app.get_description() }
            </tr>
        }
    }
}