use crate::pages::projects::content::ProjectData;
use super::*;


pub struct RPCGG;

impl ProjectData for RPCGG {
    fn project_id(&self) -> String {
        "rpc_gg".into()
    }

    fn title(&self) -> String {
        "rpc.gg".into()
    }

    fn project_type(&self) -> ProjectType {
        ProjectType::Personal
    }

    fn skills(&self) -> Vec<Skill> {
        use Skill::*;
        vec![SQL, Typescript, React, Dotnet]
    }

    fn status(&self) -> Status {
        Status::concluded((2023, 1), (2023, 12))
    }

    fn desc(&self) -> String {
        "
RPC.gg was a website to track *Friend Points*.
".into()
    }

}
