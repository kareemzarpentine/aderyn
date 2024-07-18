use std::error::Error;

use prettytable::{format, Row, Table};

use crate::{
    audit::{
        attack_surface::AttackSurfaceDetector,
        delegate_call_no_address_checks::DelegateCallNoChecksDetector,
        public_functions_no_sender::PublicFunctionsNoSenderChecksDetector,
        send_ether_no_checks::SendEtherWithoutMsgSenderChecksDetector,
    },
    context::workspace_context::WorkspaceContext,
};

pub fn get_auditor_detectors() -> Vec<Box<dyn AuditorDetector>> {
    vec![
        Box::<AttackSurfaceDetector>::default(),
        Box::<PublicFunctionsNoSenderChecksDetector>::default(),
        Box::<SendEtherWithoutMsgSenderChecksDetector>::default(),
        Box::<DelegateCallNoChecksDetector>::default(),
    ]
}

pub trait AuditorDetector: Send + Sync + 'static {
    fn detect(&mut self, _context: &WorkspaceContext) -> Result<bool, Box<dyn Error>>;

    fn title(&self) -> String;

    fn skeletal_clone(&self) -> Box<dyn AuditorDetector>;

    fn table_titles(&self) -> Row;

    fn table_rows(&self) -> Vec<Row>;
}

pub trait AuditorPrinter {
    fn print(title: &str, table_titles: Row, instances: Vec<Row>) {
        let mut table = Table::new();

        println!();
        println!("{}:", title);
        table.set_titles(table_titles);

        for instance in instances {
            table.add_row(instance);
        }

        // Set the format of the table
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
}

pub struct BasicAuditorPrinter;
impl AuditorPrinter for BasicAuditorPrinter {}
