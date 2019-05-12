#!/usr/bin/env bash

# Project definitions
EXECNAME="$(basename ${0})"
VERSION="0.1.0"

# Other useful strings
USAGE="Usage: ${EXECNAME} <config-file>
   or: ${EXECNAME} [--help,-h]
   or: ${EXECNAME} [--version,-V]
Where:
    config-file    Main configuration file containing all data needed for setup."

shopt -s extglob

# Color codes
RED=91
GREEN=32
YELLOW=33
DEFAULT_FG=39
NORMAL=0


show-usage() {
    echo "${USAGE}"
}


show-version () {
    echo "${EXECNAME} v${VERSION}"
}


show-error() {
    echo "[$(error "error")] ${1}"
}


show-action() {
    echo "[$(good "action")] ${1}"
}


kind-log() {
    echo "$(tagged ${KIND}) ${1}"
}


tagged() {
    printf "[$(warning "${1}")]"
}


bold() {
    printf "\e[1m${1}\e[22m"
}


colored() {
    printf "\e[${1}m${2}\e[${DEFAULT_FG}m"
}


good() {
    printf $(colored ${GREEN} "${1}")
}


warning() {
    printf $(colored ${YELLOW} "${1}")
}


error() {
    printf $(colored ${RED} "${1}")
}


die() {
    exit 1
}


test-args() {
    case "${1}" in
        -h | --help)
            show-usage
            exit 0
        ;; -V | --version)
            show-version
            exit 0
        ;;
    esac

    METHODS=":"

    for arg in $*
    do
        case "${arg}" in
            --full)
                METHODS=":full:"
                shift 1
            ;; --users)
                [[ ${METHODS} != ":full:" ]] && METHODS="${METHODS}users:"
                shift 1
            ;; --packages)
                [[ ${METHODS} != ":full:" ]] && METHODS="${METHODS}packages:"
                shift 1
            ;; --aur-packages)
                [[ ${METHODS} != ":full:" ]] && METHODS="${METHODS}aur-packages:"
                shift 1
            ;;
        esac
    done

    echo "Methods: ${METHODS}"

    if test -z "${1}"
    then
        show-error "Missing argument(s)."
        echo

        show-usage
        die
    fi

    export CONFIGFILE="${1}"
    export METHODS="${METHODS}"

    shift 1 # Ignores config-file

    export ARGS=$@
}

#-------------------------------------------------------------------------------
# Real commands
#-------------------------------------------------------------------------------

setup-users() {
    NAMEREGEX="[a-zA-Z]+"
    show-action "Setting up users..."
    KIND=info kind-log "Users:"

    # Parser defs
    local expected=""
    local state="none"
    local sub_state="none"
    local next_state="none"
    local user=""

    # User options
    local user_shell
    local user_groups

    reset-user-options() {
        user_shell="bash"
        user_groups=""
    }

    reset-user-options

    prepare-state() {
        next_state="$1"
        expected="="
        echo "Preparing to change to state '${next_state}'"
    }

    check-state-change() {
        if [[ "${expected}" == "${token}" ]]
        then
            case "${expected}" in
                =)
                    expected="{"
                    ;;
                {)
                    advance-state
                    ;;
            esac
        else
            show-error "Syntax-error: expected ${expected}, found ${token}."\
                && die
        fi
    }

    advance-state() {
        state="${next_state}"
        expected=""
        next_state="none"
        echo "Switched to state '${state}'"
    }

    parse-none-state() {
        if [[ -z "${expected}" ]]
        then
            case "${token}" in
                users)
                    prepare-state "users"
                    ;;
                packages)
                    prepare-state "packages"
                    ;;
                *)
                    show-error "Unexpected option '${token}'" && die
                    ;;
            esac
            return
        fi

        check-state-change
    }

    parse-users-state() {
        if [[ -z "${expected}" ]]
        then
            prepare-state "user-def"
            user="${token}"
            return
        fi

        check-state-change
    }

    parse-user-def-state() {
        if [[ -z "${expected}" ]]
        then
            option="${token}"
            expected="="
            echo "Setting option '${option}'"
            return
        fi

        case "${substate}" in
            groups)
                ;;
            none)
                if [[ "${token}" == "${expected}" ]]
                then
                    case "${expected}" in
                        =)
                            echo "option: ${option}"
                            case "${option}" in
                                groups)
                                    expected="{"
                                    ;;
                                "")
                                    echo "no option yet"
                                    ;;
                                *)
                                    expected="${NAMEREGEX}"
                                    return
                                    ;;
                            esac
                            ;;
                        "{")
                            sub_state="groups"
                            expected="${NAMEREGEX}|}"
                            ;;
                        @(${NAMEREGEX}) )
                            echo "Hehe"
                            ;;
                    esac
                else
                    if [[ "${token}" =~ ${expected} ]]
                    then
                        eval "user_${option}=${token}"
                        expected=""
                        echo "Set option '${option}' to '${token}'"
                        return
                    fi
                    show-error "Syntax-error: expected ${expected}, found ${token}."\
                        && die
                fi
                ;;
        esac
    }


    # Read the whole file
    while read -r line
    do
        for token in ${line}
        do
            KIND="${state}" kind-log "Token: ${token}"

            case "${state}" in
                none)
                    parse-none-state
                ;; users)
                    parse-users-state
                ;; user-def)
                    parse-user-def-state
                ;;
            esac
        done
    done < "${CONFIGFILE}"
}


setup-packages() {
    show-error "'setup-packages' is not implemented yet."
}


full-setup() {
    setup-users "${CONFIGFILE}" || die
    setup-packages "${CONFIGFILE}" || die
}


main() {
    test-args ${ARGS} || die

    if [[ "${METHODS}" == ":full:" ]]
    then
        full-setup
        return 0
    fi

    local methods=$(echo "${METHODS}" | sed "s/:/ /g")
    for method in ${methods}
    do
        if [[ ${method} == "users" ]]
        then
            setup-users "${CONFIGFILE}" || die
        elif [[ ${method} == "packages" ]]
        then
            setup-packages "${CONFIGFILE}" || die
        fi
    done
}


ARGS=$@ main
