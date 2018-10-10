## Dronelab

Dronelab is a wrapper and a set of Docker images to let you use [Drone CI plugins](http://plugins.drone.io/) in your Gitlab CI pipeline.

- [Dronelab](#dronelab)
- [Basic usage](#basic-usage)
  - [Providing parameters as single yaml variable](#providing-parameters-as-single-yaml-variable)
  - [Providing parameters as parameter to `dronelab` command](#providing-parameters-as-parameter-to-dronelab-command)
  - [Secrets and creating plugin variables directly](#secrets-and-creating-plugin-variables-directly)
  - [Priority of parameters](#priority-of-parameters)
- [Differences from Drone usage](#differences-from-drone-usage)
  - [Artifacts between steps](#artifacts-between-steps)
- [How it works](#how-it-works)
- [Get in touch](#get-in-touch)

## Basic usage

We'll use [Download plugin](http://plugins.drone.io/drone-plugins/drone-download/) as an example here, but the same patterns should apply to any other image. Here's the most basic use case in Drone:

```yaml
#.drone.yml
pipeline:
  download:
    image: plugins/download
    source: https://example.com/file.tar.gz
```

With Dronelab you can do something similar in your [.gitlab-ci.yml](https://docs.gitlab.com/ce/ci/yaml/) file:

```yaml
download:
  image: dronelab/download
  script: dronelab
  variables:
    source: https://example.com/file.tar.gz
```

Now this is a bit more chatty due to the way Gitlab CI pipeline is defined, but it mimics the logic close enough that you can follow the [upstream documentation](http://plugins.drone.io/drone-plugins/drone-download/) easily. Essentially the only difference is that we provide the parameters as environment variables and we use the [dronelab version of the image](https://hub.docker.com/u/dronelab/). We also need to provide a `script: dronelab` parameter because `script` is mandatory part of job definition in Gitlab CI.

This simple approach will work in most cases, however there are some exceptions. That's why there are other ways to provide the parameters with dronelab:

### Providing parameters as single yaml variable

You can provide plugin parameters as yaml string inside the `plugin` variable:

```yaml
download:
  image: dronelab/download
  script: dronelab
  variables:
    plugin: |
      source: https://example.com/file.tar.gz
```

Notice the pipe (`|`) character after `plugin:`, this way we can provide yaml as a string in the `plugin` environment variable. This format looks a bit more complicated, but it works better in some cases where using variables isn't an option - for example when you need to provide objects rather than a simple string. Let's see cloudformation plugin as an example:

```yaml
#.drone.yml
pipeline:
  deploy:
    image: robertstettner/drone-cloudformation
    stackname: my-awesome-stack
    template: templates/stack.yml
    params:
      Version: 123
      Environment: staging
```
In `.gitlab-ci.yml` file we can't define the `params` variable like that, because `variables` only accepts simple strings as value. That's were `plugin` variable can help us:

```yaml
awscf:
  image: dronelab/cloudformation
  script: dronelab
  variables:
    stackname: my-awesome-stack
    template: templates/stack.yml
    plugin: |
      params:
        Version: 123
        Environment: staging
```

⚠ Notice, that we can also combine different methods of providing the parameters.

### Providing parameters as parameter to `dronelab` command

In some cases you might not want to use the `variables` at all. One example is when you use [yaml anchors](https://docs.gitlab.com/ce/ci/yaml/#anchors) and you don't want to override `variables` set in your anchor. Instead you can also provide plugin parameters as `-p` option to the `dronelab` command:

```yaml
.awscf-template: &cftemplate
  image: dronelab/cloudformation
  variables:
    template: templates/stack.yml
    plugin: |
      params:
        Version: 123
        Environment: staging

stackone:
  <<: *cftemplate
  script: dronelab -p stackname stackone

stacktwo:
  <<: *cftemplate
  script: dronelab -p stackname stacktwo
```
Here you can see us creating yaml anchor first to create a template for `cloudformation` plugin with all the common parameters set. Then we use that anchor to define actual jobs, that each work with different stack names.

The format of the parameter is `-p <key> <value>`. You can provide it multiple times, in that case it might be easier to use yaml [folded style](http://yaml.org/spec/1.2/spec.html#id2796251) syntax:

```yaml
stackthree:
  <<: *cftemplate
  script: >
    plugin
    -p stackname stacktwo
    -p region eu-west-1
    -p mode createOrUpdate
```

⚠ Note that this example with anchors is somewhat artificially constructed, you can use [`extends` keyword](https://docs.gitlab.com/ce/ci/yaml/#extends) instead - that one is able to merge your variables.

### Secrets and creating plugin variables directly

When you define plugin parameter via `variables`, `plugin` yaml variable or as a parameter via the `-p` script argument, Dronelab will automatically [transform such variables](#how-it-works) into the `PLUGIN_VARIABLE_NAME` environment variable, because that's how Drone plugins actually work. That also means we can provide such variables directly - either via `variables` in the `.gitlab-ci.yml` file or as variables configured via CI/CD settings of the project or on the group level.

You might have noticed in the Drone plugin documentation, that some plugins accept secrets as form of configuration. These are expected in slightly different format by the Drone plugin. They are simply uppercase version of the secret name without the `PLUGIN_` prefix. Because of that you can't use `plugin` yaml variable, lowercase `variables` variable or `-p` script argument to set these as Dronelab would add the prefix to the name. In most cases you want to provide secrets as CI/CD variables defined in the CI/CD configuration rather than from the `.gitlab-ci.yml` file. (Committing secrets into your repository is generally a bad idea)

Let's look at the [Matrix plugin](http://plugins.drone.io/drone-plugins/drone-matrix/) as an example. You can provide the password either as a `password` parameter or `matrix_password` secret. That means that behind the scenes plugin expects either `PLUGIN_PASSWORD` or `MATRIX_PASSWORD` variable to be set. Let's assume we have set the password in the [Variables CI/CD configuration](https://docs.gitlab.com/ce/ci/variables/README.html#variables) for the project (or group) under the name `MATRIX_PASSWORD`.

Here's how you can send the message to the Matrix room using the saved password:

```yaml
message-one:
  image: dronelab/matrix
  script: dronelab
  variables:
    username: matrix_user
    homeserver: https://matrix.org
    roomid: abcdefghijklmnopqrstuvwxyz:matrix.org
    template: Hello from Gitlab!
```

Notice we didn't provide  the password at all! Gitlab CI will set the variable `MATRIX_PASSWORD` automatically, because it's configured in project settings. Plugin will just use it.

However if you want to be more explicit in the `.gitlab-ci.yml` file, you can explicitly provide it:

```yaml
message-two:
  image: dronelab/matrix
  script: dronelab
  variables:
    username: matrix_user
    password: $MATRIX_PASSWORD
    homeserver: https://matrix.org
    roomid: abcdefghijklmnopqrstuvwxyz:matrix.org
    template: Hello from Gitlab!
```

Sending a message to a room might be used multiple times in the pipeline, so you can either use template with yaml anchors, [`extends`](https://docs.gitlab.com/ce/ci/yaml/#extends) keyword or you can just set the variables globally so  there's only one place to change them:

```yaml
variables:
  username: matrix_user
  password: $MATRIX_PASSWORD
  homeserver: https://matrix.org
  roomid: abcdefghijklmnopqrstuvwxyz:matrix.org

message-one:
  image: dronelab/matrix
  script: dronelab
  variables:
    template: Hello from Gitlab!

message-two:
  image: dronelab/matrix
  script: dronelab
  variables:
    template: Hello again!

# You can also override some of the global variables to message to a different
# room for example.
message-elsewhere:
  image: dronelab/matrix
  script: dronelab
  variables:
    roomid: someotherroomidprovidedhere:matrix.org
    template: Hello in a different room!
```

### Priority of parameters

In case you provide the same parameter via multiple different ways, they are applied in specific order, the last one applies:

  1. Variables and secrets provided directly as `PLUGIN_VARIABLE_NAME` or `NAME_OF_SECRET`
  2. Individual variables using `variables`
  3. The `plugin` yaml variable
  4. The script parameter via `-p <key> <value>`

That means the following configuration will download version 1 of the file:

```yaml
download:
  image: dronelab/download
  script: dronelab -p source https://example.com/file_version_1.gz
  variables:
    source: https://example.com/file_version_2.gz
    plugin: |
      source: https://example.com/file_version_3.gz
```

This would download version 3:

```yaml
download:
  image: dronelab/download
  script: dronelab
  variables:
    source: https://example.com/file_version_2.gz
    plugin: |
      source: https://example.com/file_version_3.gz 
```

## Differences from Drone usage

### Artifacts between steps

In Drone all plugins work on top of the same workspace, so for example files downloaded by `download` plugin are automatically available to the next steps in pipeline. If you have Gitlab cache enabled globally, it behaves in similar fashion, however if you have caches disabled you might need to explicitly configure `cache` or `artifacts` (whatever feels more appropriate) to pass the generated content to next job.

## How it works

Dronelab plugins should work with any executor, that [supports the `image` keyword](https://docs.gitlab.com/runner/executors/README.html#compatibility-chart). Currently that means you can use the Docker, Docker Machine, Docker Machine SSH and Kubernetes executors.

Dronelab plugin image is created by taking the upstream plugin image and wrapping it in `alpine` image with `dronelab` binary added. The alpine image is used as Gitlab CI needs something capable running bash scripts.

The `dronelab` wrapper then reads provided parameters and environment variables and attempts to translate those to the format expected by Drone plugins. It also attempts to translate [predefined variables](https://docs.gitlab.com/ce/ci/variables/README.html#predefined-variables-environment-variables) to the format expected by Drone.

When all variables are created, dronelab will try to execute binary defined in `DRONELAB` environment variable. That one is usually already defined in the Dronelab plugin image and points to the upstream plugin binary.

⚠ Note, that not all variables that are normally available in Drone build environment are currently available in the Dronelab environment. Some are not provided by Gitlab CI, some don't even have their counterpart in the Gitlab word. Some variables have slightly different format. Most plugins use just very small subset of these variables and hence most should work just fine.

## Get in touch

To report a bug or ask a question, please [open an issue](https://github.com/mprasil/dronelab/issues/new).