FROM plugins/npm as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="plugins/npm"
ENV DRONELAB_UPSTREAM_AUTHOR="donny-dont"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/drone-plugins/drone-npm"
ENV DRONELAB="/bin/drone-npm"
