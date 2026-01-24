from __future__ import annotations

from pathlib import Path


def _read_sitemap_bytes(site_dir: Path) -> bytes | None:
    """
    MkDocs generates `sitemap.xml` only once (often only for the default build).

    Material's language switcher JS attempts to fetch `sitemap.xml` relative to
    the current page URL, which can be nested. To avoid 404s, we copy the
    sitemap to every page directory after build.
    """

    direct = site_dir / "sitemap.xml"
    if direct.exists():
        return direct.read_bytes()

    sibling = site_dir.parent / "sitemap.xml"
    if sibling.exists():
        return sibling.read_bytes()

    return None


def _read_sitemap_gz_bytes(site_dir: Path) -> bytes | None:
    direct = site_dir / "sitemap.xml.gz"
    if direct.exists():
        return direct.read_bytes()

    sibling = site_dir.parent / "sitemap.xml.gz"
    if sibling.exists():
        return sibling.read_bytes()

    return None


def on_post_build(config) -> None:
    site_dir = Path(config["site_dir"]).resolve()
    sitemap_bytes = _read_sitemap_bytes(site_dir)
    if sitemap_bytes is None:
        return

    sitemap_gz_bytes = _read_sitemap_gz_bytes(site_dir)

    for index_path in site_dir.rglob("index.html"):
        page_dir = index_path.parent

        sitemap_target = page_dir / "sitemap.xml"
        if not sitemap_target.exists():
            sitemap_target.write_bytes(sitemap_bytes)

        if sitemap_gz_bytes is not None:
            sitemap_gz_target = page_dir / "sitemap.xml.gz"
            if not sitemap_gz_target.exists():
                sitemap_gz_target.write_bytes(sitemap_gz_bytes)
