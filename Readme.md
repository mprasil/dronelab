## Dronelab

Dronelab is a wrapper and a set of Docker images to let you use [Drone CI plugins](http://plugins.drone.io/) in your Gitlab CI pipeline.

- [Dronelab](#dronelab)
- [Usage](#usage)
- [Differences from Drone usage](#differences-from-drone-usage)
- [How it works](#how-it-works)
- [Get in touch](#get-in-touch)

## Usage

We'll use [Download plugin](http://plugins.drone.io/drone-plugins/drone-download/) as an example here, but the same patterns should apply to any other image. Here's the most basic use case in Drone:

```yaml
pipeline:
  download:
    image: plugins/download
    source: https://example.com/file.tar.gz
```

With Dronelab you can do something similar in your [.gitlab-ci.yml](https://docs.gitlab.com/ce/ci/yaml/) file:

```yaml
download:
  image: dronelab/download
  environment:
    plugin: |
      source: https://example.com/file.tar.gz
  script: plugin
```

Now this is a bit more chatty due to the way Gitlab CI pipeline is defined, but it mimics the logic close enough that you can follow the upstream documentation easily.


## Differences from Drone usage

## How it works

## Get in touch

To report a bug or ask a question, please open an issue.