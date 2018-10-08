FROM gboo/github-search-downstream as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="gboo/github-search-downstream"
ENV DRONELAB_UPSTREAM_AUTHOR="gboddin"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/gboddin/drone-github-search-downstream"
ENV DRONELAB="/bin/github-search-downstream"
