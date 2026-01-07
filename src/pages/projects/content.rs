use chrono::NaiveDate;

use crate::{
    components::{Listable, Listing, navbar::Route},
    data::{markdown::Markdown, skill::Skill},
    pages::projects::content::{rpc_bot::RPCBot, rpc_gg::RPCGG, test::TestProject},
};

#[derive(Clone)]
pub struct Project {
    project_id: String,
    pub title: String,
    pub link: Option<String>,
    pub project_type: ProjectType,
    pub skills: Vec<Skill>,
    pub status: Status,
    pub desc: Markdown,
}

impl Project {
    pub fn id(&self) -> String {
        self.project_id.clone()
    }

    pub fn get_listing(project_data: impl ProjectData) -> (Listing, Vec<Skill>) {
        let project_id = project_data.project_id();
        let title = project_data.title();
        let skills = project_data.skills();

        let lisitng = Listing::new(
            project_id.as_str().into(),
            title,
            Route::ProjectContent { id: project_id },
        );

        (lisitng, skills)
    }

    pub fn from_project_data(project_data: impl ProjectData) -> Self {
        let project_id = project_data.project_id();
        let title = project_data.title();
        let link = project_data.link();
        let project_type = project_data.project_type();
        let skills = project_data.skills();
        let status = project_data.status();
        let desc = Markdown::from_string(project_data.desc());

        Self {
            project_id,
            desc,
            title,
            link,
            project_type,
            skills,
            status,
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
    fn project_id(&self) -> String;

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

#[derive(Clone, PartialEq)]
pub struct ProjectListings;

fn to_listing<T: ProjectData>(projects: Vec<T>) -> Vec<Listing> {
    let route = |p: &T| Route::ProjectContent { id: p.project_id() };

    let listings: Vec<Listing> = projects
        .iter()
        .map(|p| Listing::new(p.project_id().as_str().into(), p.title(), route(p)))
        .collect();

    listings
}

pub fn project_listings() -> Vec<(Listing, Vec<Skill>)> {
    vec![
        Project::get_listing(RPCBot),
        Project::get_listing(RPCGG),
        Project::get_listing(TestProject)]
}

pub fn projects() -> Vec<Project> {
    vec![Project::from_project_data(RPCBot), Project::from_project_data(RPCGG),
        Project::from_project_data(TestProject)]
}

pub fn get_project_by_id(id: String) -> Option<Project> {
    projects().iter().find(|x| x.id() == id).cloned()
}

mod rpc_bot;
mod rpc_gg;
mod test;
