FROM jmccann/drone-clair:1 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="jmccann/drone-clair:1"
ENV DRONELAB_UPSTREAM_AUTHOR="jmccann"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/jmccann/drone-clair"
ENV DRONELAB="/bin/drone-clair"
