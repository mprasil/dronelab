use std::collections::HashMap;
use envsource::*;

pub fn drone_envs() -> HashMap<String, String> {
    let mut envs: HashMap<&str, Option<String>> = HashMap::new();

    envs.insert("CI", EnvDefault::define("CI_DRONE", "drone").get());
    envs.insert("DRONE", EnvDefault::define("DRONE", "true").get());
    envs.insert("DRONE_REPO", EnvSkip::define("CI_PROJECT_PATH").get()); //repository full name
    envs.insert("DRONE_REPO_OWNER",EnvSkip::define("CI_PROJECT_NAME").get()); // repository owner
    envs.insert("DRONE_REPO_NAME",EnvSkip::define("CI_PROJECT_NAME").get()); // repository name
    envs.insert("DRONE_REPO_LINK",EnvSkip::define("CI_PROJECT_URL").get()); // repository link

    envs.insert("DRONE_REPO_BRANCH", EnvSkip::define("DRONE_REPO_BRANCH").get()); // repository default branch
    envs.insert("DRONE_REPO_PRIVATE", EnvDefault::define(
        "DRONE_REPO_PRIVATE",
        match EnvDefault::define("CI_PROJECT_VISIBILITY","private").get().unwrap().as_str() {
            "public" => "false",
            _ => "true"
        }
        ).get()); // repository is private
    envs.insert("DRONE_REPO_TRUSTED", EnvSkip::define("DRONE_REPO_TRUSTED").get()); // repository is trusted
    envs.insert("DRONE_REMOTE_URL",EnvSkip::define("CI_REPOSITORY_URL").get()); // git remote url
    envs.insert("DRONE_COMMIT_SHA",EnvSkip::define("CI_COMMIT_SHA").get()); // git commit sha
    envs.insert("DRONE_COMMIT_MESSAGE",EnvSkip::define("CI_COMMIT_MESSAGE").get()); // git commit message
    envs.insert("DRONE_COMMIT_LINK",EnvDefault::define(
        "DRONE_COMMIT_LINK",
        format!("{}/commit/{}",
            EnvDefault::define("CI_PROJECT_URL","").get().unwrap(),
            EnvDefault::define("CI_COMMIT_SHA","").get().unwrap(),
            ).as_str()
        ).get()); // git commit link
    envs.insert("DRONE_COMMIT_AUTHOR",EnvSkip::define("DRONE_COMMIT_AUTHOR").get()); // git author name
    envs.insert("DRONE_COMMIT_AUTHOR_EMAIL",EnvSkip::define("DRONE_COMMIT_AUTHOR_EMAIL").get()); // git author email
    envs.insert("DRONE_COMMIT_AUTHOR_AVATAR",EnvSkip::define("DRONE_COMMIT_AUTHOR_AVATAR").get()); // git author avatar
    envs.insert("DRONE_COMMIT_REF",EnvDefault::define(
        "DRONE_COMMIT_REF",
        format!("refs/heads/{}", EnvDefault::define("CI_COMMIT_REF_NAME","master").get().unwrap()).as_str()
        ).get()); // git commit ref
    envs.insert("DRONE_COMMIT_BRANCH",EnvSkip::define("CI_COMMIT_REF_NAME").get()); // git commit branch

    envs.insert("DRONE_BUILD_NUMBER",EnvSkip::define("CI_PIPELINE_IID").get()); // build number
    envs.insert("DRONE_BUILD_CREATED",EnvSkip::define("DRONE_BUILD_CREATED").get()); // build created
    envs.insert("DRONE_BUILD_STARTED",EnvSkip::define("DRONE_BUILD_STARTED").get()); // build started
    envs.insert("DRONE_BUILD_FINISHED",EnvSkip::define("DRONE_BUILD_FINISHED").get()); // build finished
    envs.insert("DRONE_BUILD_LINK",EnvSkip::define("CI_PIPELINE_URL").get()); // build link
    envs.insert("DRONE_DEPLOY_TO",EnvSkip::define("CI_ENVIRONMENT_NAME").get()); // build deployment target
    envs.insert("DRONE_YAML_VERIFIED",EnvSkip::define("DRONE_YAML_VERIFIED").get()); // build yaml is verified
    envs.insert("DRONE_YAML_SIGNED",EnvSkip::define("DRONE_YAML_SIGNED").get()); // build yaml is signed
    envs.insert("DRONE_BUILD_STATUS",EnvDefault::define("DRONE_BUILD_STATUS", "success").get()); // build status
    envs.insert("DRONE_BUILD_EVENT",EnvDefault::define("DRONE_BUILD_EVENT", "push").get()); // build event
    envs.insert("DRONE_PREV_BUILD_NUMBER",EnvSkip::define("DRONE_PREV_BUILD_NUMBER").get()); // previous build number
    envs.insert("DRONE_PREV_BUILD_STATUS",EnvSkip::define("DRONE_PREV_BUILD_STATUS").get()); // previous build status
    envs.insert("DRONE_PREV_COMMIT_SHA",EnvSkip::define("CI_COMMIT_BEFORE_SHA").get()); // previous build sha

    envs.iter()
    .filter(|&(_,v)| v.is_some())
    .map(|(k,v)| (k.to_string(),v.clone().unwrap()))
    .collect()
}