FROM aosapps/drone-sonar-plugin as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="aosapps/drone-sonar-plugin"
ENV DRONELAB_UPSTREAM_AUTHOR="aosapps"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/aosapps/drone-sonar-plugin"
ENV DRONELAB="/bin/sh -c /bin/drone-sonar"
