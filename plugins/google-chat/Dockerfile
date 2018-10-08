FROM peloton/drone-google-chat as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="peloton/drone-google-chat"
ENV DRONELAB_UPSTREAM_AUTHOR="josmo"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/josmo/drone-google-chat"
ENV DRONELAB="/bin/drone-google-chat"
