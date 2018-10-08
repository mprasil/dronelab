FROM cbrgm/drone-hugo as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="cbrgm/drone-hugo"
ENV DRONELAB_UPSTREAM_AUTHOR="cbrgm"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/cbrgm/drone-hugo"
ENV DRONELAB="/bin/sh -c /bin/sh /bin/drone-hugo.sh"
