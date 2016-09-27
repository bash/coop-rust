#!/bin/sh

_COOP_CACHE_FILE="${HOME}/.coop-locations-cache"

_coop_locations() {
  coop locations \
    | grep -e  "- *" \
    | sed -e 's/\-//g'
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
    _coop_locations | tee ${_COOP_CACHE_FILE}
  fi
}

_coop () {
  local cur opts

  COMPREPLY=()
  cur=${COMP_WORDS[COMP_CWORD]}
  prev=${COMP_WORDS[COMP_CWORD-1]}

  opts="menus locations dish-stats"

  if [ "${prev}" = "menus" ]; then
    COMPREPLY=( $(compgen -W "${prev} $(_coop_cached_locations)" -- ${cur}) )
  else
    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
  fi

  return 0
}

complete -F _coop -o filenames coop