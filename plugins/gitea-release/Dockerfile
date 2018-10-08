FROM plugins/gitea-release as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="plugins/gitea-release"
ENV DRONELAB_UPSTREAM_AUTHOR="drone-plugins"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/drone-plugins/drone-gitea-release"
ENV DRONELAB="/bin/drone-gitea-release"
