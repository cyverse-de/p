(defproject org.cyverse/cyverse-de-protobufs "0.0.6-SNAPSHOT"
  :description "Code generated from the DE protocol buffer definitions"
  :url "https://github.com/cyverse-de/p"
  :license {:name "BSD"
            :url "https://cyverse.org/license"}
  :dependencies [[org.clojure/clojure "1.12.0"]
                 [clojusc/protobuf "3.5.1-v1.1"]
                 [com.google.protobuf/protobuf-java "4.29.3"]
                 [com.google.protobuf/protobuf-java-util "4.29.3"]]
  :plugins [[lein-ancient "0.7.0"]]
  :java-source-paths ["java"]
  :deploy-repositories [["releases" :clojars]
                        ["snapshots" :clojars]]
  :release-tasks [["vcs" "assert-committed"]
                  ["change" "version" "leiningen.release/bump-version" "release"]
                  ["vcs" "commit" "java/v%s release"]
                  ["vcs" "tag" "java/v"]
                  ["deploy"]
                  ["change" "version" "leiningen.release/bump-version"]
                  ["vcs" "commit" "Bump java version to %s"]
                  ["vcs" "push"]])
