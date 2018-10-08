FROM joshdvir/drone-ecs-deploy as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="joshdvir/drone-ecs-deploy"
ENV DRONELAB_UPSTREAM_AUTHOR="devops-israel"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/joshdvir/drone-ecs-deploy"
ENV DRONELAB="/bin/bash"
