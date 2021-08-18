with import <nixpkgs> {};
let
  gst-plugins-base = gst_all_1.gst-plugins-base.override {
    enableCocoa = stdenv.isDarwin;
  };
in
mkShell {
  name = "gst-meet";
  buildInputs = [
    pkg-config
    glib
    glib-networking
    gst_all_1.gstreamer
    gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
    libnice
  ] ++ (if stdenv.isDarwin then [
    darwin.apple_sdk.frameworks.AppKit
    darwin.apple_sdk.frameworks.Security
  ] else []);

  GIO_EXTRA_MODULES = ["${pkgs.glib-networking.out}/lib/gio/modules"];
}
