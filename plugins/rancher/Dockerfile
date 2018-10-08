FROM peloton/drone-rancher as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="peloton/drone-rancher"
ENV DRONELAB_UPSTREAM_AUTHOR="josmo"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/josmo/drone-rancher"
ENV DRONELAB="/bin/drone-rancher"
