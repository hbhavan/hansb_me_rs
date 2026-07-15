use core::option::Option::None;
use std::ops::{Deref};

use chrono::NaiveDate;
use dioxus::prelude::*;

use writ::data::{markdown::*};
use crate::{
    components::{Listable, Listing},
    data::{searchable::{SearchItem, Searchable}, skill::Skill},
    layout::Route,
    pages::projects::content::{
        rpc_bot::RPCBot,
        rpc_gg::RPCGG,
        set_game::{SetGame},
        test::TestProject},
};

#[derive(Clone)]
pub struct Project {
    project_id: i32,
    pub title: String,
    pub link: Option<String>,
    pub project_type: ProjectType,
    pub skills: Vec<Skill>,
    pub status: Status,
    pub desc: MarkdownContent,
    pub content: Option<Element>
}

impl Project {
    pub fn id(&self) -> i32 {
        self.project_id.clone()
    }

    pub fn get_listing(project_data: &dyn ProjectData) -> (Listing, Vec<Skill>) {
        let project_id = project_data.project_id();
        let title = project_data.title();
        let skills = project_data.skills();

        let listing = Listing::new(
            project_id,
            title,
            Route::ProjectContent { id: project_id },
        );

        (listing, skills)
    }

    pub fn from_project_data(project_data: &dyn ProjectData) -> Self {
        let project_id = project_data.project_id();
        let title = project_data.title();
        let link = project_data.link();
        let project_type = project_data.project_type();
        let skills = project_data.skills();
        let status = project_data.status();
        let desc = MarkdownContent::from_string(&project_data.desc());
        let content = project_data.render_project();

        Self {
            project_id,
            desc,
            title,
            link,
            project_type,
            skills,
            status,
            content
        }
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.project_id == other.project_id
    }
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ProjectType {
    Personal,
    Professional,
    WIP,
    Experiment,
    Misc,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Ongoing(NaiveDate),
    Concluded(NaiveDate, NaiveDate),
    Unknown,
}

pub trait ProjectData {
    fn project_id(&self) -> i32;

    fn title(&self) -> String;

    fn link(&self) -> Option<String> {
        None
    }

    fn project_type(&self) -> ProjectType;

    fn skills(&self) -> Vec<Skill> {
        vec![]
    }

    fn status(&self) -> Status;

    fn desc(&self) -> String;

    fn render_project(&self) -> Option<Element> { None }
}

impl Status {
    pub fn ongoing(year: i32, month: u32) -> Status {
        match NaiveDate::from_ymd_opt(year, month, 1) {
            Some(dt) => Status::Ongoing(dt),
            None => Status::Unknown,
        }
    }

    pub fn concluded(start: (i32, u32), end: (i32, u32)) -> Status {
        match (
            NaiveDate::from_ymd_opt(start.0, start.1, 1),
            NaiveDate::from_ymd_opt(end.0, end.1, 1),
        ) {
            (Some(dt1), Some(dt2)) => Status::Concluded(dt1, dt2),
            (Some(dt1), None) => Status::Ongoing(dt1),
            _ => Status::Unknown,
        }
    }
}

impl Listable for (Listing, Vec<Skill>) {
    fn key(&self) -> String {
        self.0.key()
    }

    fn to_listing(&self) -> Listing {
        self.0.clone()
    }
}

impl SearchItem for (Listing, Vec<Skill>) {
    fn text(&self) -> &String {
        &self.0.title
    }

    fn value(&self) -> String {
        self.0.id().to_string()
    }
}

impl Searchable for (Listing, Vec<Skill>) { }

pub fn projects() -> Vec<Box<dyn ProjectData>> {
    vec![
        Box::new(RPCBot),
        Box::new(RPCGG),
        Box::new(TestProject),
        Box::new(SetGame),
    ]
}

pub fn project_listings() -> Vec<(Listing, Vec<Skill>)> {
    projects()
        .iter()
        .map(|x| Project::get_listing(x.deref()))
        .collect()
}


pub fn get_project_by_id(id: i32) -> Option<Project> {
    projects()
        .iter()
        .find(|x| x.project_id() == id)
        .map(|x| Project::from_project_data(x.deref()))
}

mod rpc_bot;
mod rpc_gg;
pub mod set_game;
mod test;
