FROM jonasfranz/crowdin as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="jonasfranz/crowdin"
ENV DRONELAB_UPSTREAM_AUTHOR="JonasFranzDEV"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/JonasFranzDEV/drone-crowdin"
ENV DRONELAB="/bin/drone-crowdin"
