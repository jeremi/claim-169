"""MkDocs hooks for post-build processing."""

import shutil
from pathlib import Path


def on_post_build(config, **kwargs):
    """Copy sitemap.xml to all subdirectories.

    MkDocs Material's instant navigation feature requests sitemap.xml
    relative to the current URL path. This hook ensures every directory
    has a copy of the sitemap to prevent 404 errors.
    """
    site_dir = Path(config["site_dir"])
    sitemap_path = site_dir / "sitemap.xml"
    sitemap_gz_path = site_dir / "sitemap.xml.gz"

    if not sitemap_path.exists():
        return

    # Copy sitemap to all subdirectories that contain index.html (actual pages)
    for subdir in site_dir.rglob("*"):
        if subdir.is_dir() and (subdir / "index.html").exists():
            target = subdir / "sitemap.xml"
            if not target.exists():
                shutil.copy(sitemap_path, target)

            if sitemap_gz_path.exists():
                target_gz = subdir / "sitemap.xml.gz"
                if not target_gz.exists():
                    shutil.copy(sitemap_gz_path, target_gz)
