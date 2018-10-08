FROM drillster/drone-email as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="drillster/drone-email"
ENV DRONELAB_UPSTREAM_AUTHOR="drillster"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/drillster/drone-email"
ENV DRONELAB="/bin/drone-email"
