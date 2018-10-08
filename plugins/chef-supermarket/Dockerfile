FROM jmccann/drone-chef-supermarket:1 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="jmccann/drone-chef-supermarket:1"
ENV DRONELAB_UPSTREAM_AUTHOR="jmccann"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/jmccann/drone-chef-supermarket"
ENV DRONELAB="/usr/bin/drone-chef-supermarket upload"
