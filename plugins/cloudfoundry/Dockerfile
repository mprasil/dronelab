FROM cheslip/drone-cloudfoundry as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="cheslip/drone-cloudfoundry"
ENV DRONELAB_UPSTREAM_AUTHOR="cheslip"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/Comcast/drone-cloudfoundry"
ENV DRONELAB="/bin/sh -c /bin/deploy.sh"
