(defproject org.cyverse/cyverse-de-protobufs "0.0.2"
  :description "Code generated from the DE protocol buffer definitions"
  :url "https://github.com/cyverse-de/p"
  :license {:name "BSD"
            :url "https://cyverse.org/license"}
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [clojusc/protobuf "3.5.1-v1.1"]
                 [com.google.protobuf/protobuf-java "3.25.3"]]
  :plugins [[lein-ancient "0.7.0"]]
  :java-source-paths ["java"])
