mod view;

use commander_tournament_core::tournament::Tournament;
use iced::Element;
use iced_tea::{Component, Model, Signal};

use crate::{
    components::{Paginator, PaginatorMsg},
    models::{
        file::{FileModel, FileModelMsg},
        tournament::TournamentModel,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::VariantArray, derive_more::Display)]
pub enum ImportOrder {
    #[default]
    #[display("Append After Existing Games")]
    Append,
    #[display("Prepend Before Existing Games")]
    Prepend,
}

#[derive(Debug, Default)]
pub struct ImportState {
    source: FileModel<Tournament>,
    order: ImportOrder,
    paginator: Paginator,
    error: Option<String>,
}

#[derive(Debug, Clone, derive_more::From)]
pub enum ImportMsg {
    OpenFile,
    ClearFile,
    File(FileModelMsg<Tournament>),
    #[from(ignore)]
    SetOrder(ImportOrder),
    Paginator(PaginatorMsg),
    Apply,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum ImportOut {
    Apply { source: Box<Tournament>, prepend: bool },
    Close,
}

impl Model for ImportState {
    type Message = ImportMsg;
    type OutMessage = ImportOut;
    type Context<'a>
        = &'a TournamentModel
    where
        Self: 'a;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        _context: Self::Context<'a>,
    ) -> anyhow::Result<Signal<Self::Message, Self::OutMessage>> {
        match message {
            ImportMsg::OpenFile => {
                self.error = None;
                self.source.map_update(FileModelMsg::Open, (), |_| Signal::done())
            }
            ImportMsg::ClearFile => {
                self.error = None;
                self.paginator.reset();
                self.source
                    .map_update(FileModelMsg::Close, (), |_| Signal::done())
            }
            ImportMsg::File(file_msg) => {
                self.error = None;
                match self.source.map_update(file_msg, (), |_| Signal::done()) {
                    Ok(sig) => {
                        self.paginator.reset();
                        Ok(sig)
                    }
                    Err(err) => {
                        self.error = Some(err.to_string());
                        Signal::done()
                    }
                }
            }
            ImportMsg::SetOrder(order) => {
                self.order = order;
                Signal::done()
            }
            ImportMsg::Paginator(msg) => self.paginator.empty_update(msg, 0),
            ImportMsg::Apply => {
                let Some(imported) = self.source.data().cloned() else {
                    return Signal::done();
                };
                let prepend = self.order == ImportOrder::Prepend;
                self.paginator.reset();
                self.error = None;
                Ok(Signal::out(ImportOut::Apply {
                    source: Box::new(imported),
                    prepend,
                })
                .chain(Signal::msg(ImportMsg::ClearFile)))
            }
            ImportMsg::Cancel => {
                self.paginator.reset();
                self.error = None;
                Ok(Signal::msg(ImportMsg::ClearFile).chain(Signal::out(ImportOut::Close)))
            }
        }
    }
}

impl Component for ImportState {
    fn render<'a>(&'a self, context: Self::Context<'a>) -> Element<'a, Self::Message> {
        view::render_import(self, context)
    }
}
