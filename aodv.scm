(use-modules (guix packages)
             (guix download)
             (guix build-system cargo)
             (guix licenses))

(package
 (name "aodv")
 (version "0.1")
 (source (origin
          (method url-fetch)
          (uri (string-append "" version ".tar.gz"))))
 (build-system cargo-build-system)
 (synopsis "aodv:  Ad Hoc Distance Vector based link Protocol")
 (description "")
 (home-page "")
 (license mit))
