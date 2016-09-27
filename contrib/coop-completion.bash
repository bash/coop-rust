#!/bin/sh

_COOP_CACHE_FILE="${HOME}/.coop-locations-cache"

_coop_get_locations() {
  coop locations \
    | grep -e  "- *" \
    | sed -e 's/\-//g'
}

_coop_lowercase () {
    echo ${1} | tr '[:upper:]' '[:lower:]'
}

_coop_cached_locations() {
  local modified locations
  modified=$(date -r ${_COOP_CACHE_FILE} +%s 2> /dev/null)
  now=$(date "+%s")
  diff=$(expr ${modified} - ${now} 2> /dev/null)

  # delete cached locations after 24 hours
  if [ -f ${_COOP_CACHE_FILE} ] && [ ${diff} -gt 86400 ]; then
    rm -f ${_COOP_CACHE_FILE}
  fi

  if [ -f ${_COOP_CACHE_FILE} ]; then
    cat ${_COOP_CACHE_FILE}
  else
    touch ${_COOP_CACHE_FILE}
    _coop_get_locations | tee ${_COOP_CACHE_FILE}
  fi
}

_coop_locations() {
    local cur loc locations lower_loc

    locations=$(_coop_cached_locations)
    cur=$(_coop_lowercase ${COMP_WORDS[COMP_CWORD]})

    for loc in ${locations}; do
        lower_loc=$(_coop_lowercase "$loc")

        if [[ "$lower_loc" == "$cur"* ]]; then
            COMPREPLY+=("$loc")
        fi
    done
}

_coop () {
  local cur opts

  COMPREPLY=()
  cur=${COMP_WORDS[COMP_CWORD]}
  cur_lower=$(echo ${cur} | tr '[:upper:]' '[:lower:]')
  prev=${COMP_WORDS[COMP_CWORD-1]}

  opts="menus locations dish-stats"

  if [ "${prev}" = "menus" ]; then
    _coop_locations
  else
    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
  fi

  return 0
}

complete -F _coop -o filenames coop
