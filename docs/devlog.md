# Dev Log

## Foreword

To try and keep the motivation high and improve project transparecy, I am
starting a development log. This document contains some information about the
development sessions of the authors. In case this project becomes actually
bigger, this will likely get archived as impractical to maintain, but while it's
just me, I think it makes sense.

The first entry in this logbook is not the first session of developing this
project.

Also, since I'm a night person, I will count sessions continuing into the next
day as part of the previous one. It's just more convenient that way.

## 2025-09-15 (plexsheep)

### Progress

- Slop specification was written
- Removed non-critical stuff from the Readme, and text is now all authentic
- Fancy network stack diagram for spec
- Create identity in gui #4

### Decisions

- Rebranded to SREMP because GRRSMP was way too unprofessional, and this project is weirdly important to me
- GitHub over git.cscherr.de (forgejo), for amazing CI infrastructure and discoverability
- Removed version and multi_device flag from identity in spec

### Notes

- Working with a specification is awesome actually! I can define ideas with natural language and refer to them without needing to code them first.
- It seems like we have some deadlocks in the application... Some may be from GTK, and some may be, even worse, from the core state.

### Mood

Motivation high, I'm not just writing git commits anymore. I am
_committing_. Maybe.
