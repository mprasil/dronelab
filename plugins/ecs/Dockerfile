FROM peloton/drone-ecs as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="peloton/drone-ecs"
ENV DRONELAB_UPSTREAM_AUTHOR="josmo"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/josmo/drone-ecs"
ENV DRONELAB="/bin/drone-ecs"
