FROM gjtempleton/drone-pypi as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="gjtempleton/drone-pypi"
ENV DRONELAB_UPSTREAM_AUTHOR="gjtempleton"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/gjtempleton/drone-pypi"
ENV DRONELAB="/bin/drone-pypi"
