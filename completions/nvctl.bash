_nvctl() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="nvctl"
                ;;
            nvctl,arch)
                cmd="nvctl__subcmd__arch"
                ;;
            nvctl,asus)
                cmd="nvctl__subcmd__asus"
                ;;
            nvctl,color)
                cmd="nvctl__subcmd__color"
                ;;
            nvctl,companion)
                cmd="nvctl__subcmd__companion"
                ;;
            nvctl,completion)
                cmd="nvctl__subcmd__completion"
                ;;
            nvctl,config)
                cmd="nvctl__subcmd__config"
                ;;
            nvctl,container)
                cmd="nvctl__subcmd__container"
                ;;
            nvctl,display)
                cmd="nvctl__subcmd__display"
                ;;
            nvctl,dlss)
                cmd="nvctl__subcmd__dlss"
                ;;
            nvctl,doctor)
                cmd="nvctl__subcmd__doctor"
                ;;
            nvctl,driver)
                cmd="nvctl__subcmd__driver"
                ;;
            nvctl,fan)
                cmd="nvctl__subcmd__fan"
                ;;
            nvctl,gaming)
                cmd="nvctl__subcmd__gaming"
                ;;
            nvctl,gpu)
                cmd="nvctl__subcmd__gpu"
                ;;
            nvctl,hdr)
                cmd="nvctl__subcmd__hdr"
                ;;
            nvctl,help)
                cmd="nvctl__subcmd__help"
                ;;
            nvctl,interactive)
                cmd="nvctl__subcmd__interactive"
                ;;
            nvctl,kde)
                cmd="nvctl__subcmd__kde"
                ;;
            nvctl,monitor)
                cmd="nvctl__subcmd__monitor"
                ;;
            nvctl,monitors)
                cmd="nvctl__subcmd__monitors"
                ;;
            nvctl,nvtop)
                cmd="nvctl__subcmd__nvtop"
                ;;
            nvctl,osd)
                cmd="nvctl__subcmd__osd"
                ;;
            nvctl,overclock)
                cmd="nvctl__subcmd__overclock"
                ;;
            nvctl,passthrough)
                cmd="nvctl__subcmd__passthrough"
                ;;
            nvctl,power)
                cmd="nvctl__subcmd__power"
                ;;
            nvctl,power-profile)
                cmd="nvctl__subcmd__power__subcmd__profile"
                ;;
            nvctl,recording)
                cmd="nvctl__subcmd__recording"
                ;;
            nvctl,shaders)
                cmd="nvctl__subcmd__shaders"
                ;;
            nvctl,system)
                cmd="nvctl__subcmd__system"
                ;;
            nvctl,tui)
                cmd="nvctl__subcmd__tui"
                ;;
            nvctl,upscaling)
                cmd="nvctl__subcmd__upscaling"
                ;;
            nvctl,version)
                cmd="nvctl__subcmd__version"
                ;;
            nvctl,vibrance)
                cmd="nvctl__subcmd__vibrance"
                ;;
            nvctl,vrr)
                cmd="nvctl__subcmd__vrr"
                ;;
            nvctl,wayland)
                cmd="nvctl__subcmd__wayland"
                ;;
            nvctl__subcmd__arch,aur-suggestions)
                cmd="nvctl__subcmd__arch__subcmd__aur__subcmd__suggestions"
                ;;
            nvctl__subcmd__arch,check-updates)
                cmd="nvctl__subcmd__arch__subcmd__check__subcmd__updates"
                ;;
            nvctl__subcmd__arch,help)
                cmd="nvctl__subcmd__arch__subcmd__help"
                ;;
            nvctl__subcmd__arch,install-hooks)
                cmd="nvctl__subcmd__arch__subcmd__install__subcmd__hooks"
                ;;
            nvctl__subcmd__arch,mkinitcpio)
                cmd="nvctl__subcmd__arch__subcmd__mkinitcpio"
                ;;
            nvctl__subcmd__arch,rebuild-dkms)
                cmd="nvctl__subcmd__arch__subcmd__rebuild__subcmd__dkms"
                ;;
            nvctl__subcmd__arch,remove-hooks)
                cmd="nvctl__subcmd__arch__subcmd__remove__subcmd__hooks"
                ;;
            nvctl__subcmd__arch,status)
                cmd="nvctl__subcmd__arch__subcmd__status"
                ;;
            nvctl__subcmd__arch__subcmd__help,aur-suggestions)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__aur__subcmd__suggestions"
                ;;
            nvctl__subcmd__arch__subcmd__help,check-updates)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__check__subcmd__updates"
                ;;
            nvctl__subcmd__arch__subcmd__help,help)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__arch__subcmd__help,install-hooks)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__install__subcmd__hooks"
                ;;
            nvctl__subcmd__arch__subcmd__help,mkinitcpio)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__mkinitcpio"
                ;;
            nvctl__subcmd__arch__subcmd__help,rebuild-dkms)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__rebuild__subcmd__dkms"
                ;;
            nvctl__subcmd__arch__subcmd__help,remove-hooks)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__remove__subcmd__hooks"
                ;;
            nvctl__subcmd__arch__subcmd__help,status)
                cmd="nvctl__subcmd__arch__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__asus,aura)
                cmd="nvctl__subcmd__asus__subcmd__aura"
                ;;
            nvctl__subcmd__asus,detect)
                cmd="nvctl__subcmd__asus__subcmd__detect"
                ;;
            nvctl__subcmd__asus,help)
                cmd="nvctl__subcmd__asus__subcmd__help"
                ;;
            nvctl__subcmd__asus,power)
                cmd="nvctl__subcmd__asus__subcmd__power"
                ;;
            nvctl__subcmd__asus,status)
                cmd="nvctl__subcmd__asus__subcmd__status"
                ;;
            nvctl__subcmd__asus__subcmd__aura,color)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__color"
                ;;
            nvctl__subcmd__asus__subcmd__aura,help)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help"
                ;;
            nvctl__subcmd__asus__subcmd__aura,mode)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__mode"
                ;;
            nvctl__subcmd__asus__subcmd__aura,restore)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__restore"
                ;;
            nvctl__subcmd__asus__subcmd__aura,status)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__status"
                ;;
            nvctl__subcmd__asus__subcmd__aura,temp-reactive)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__temp__subcmd__reactive"
                ;;
            nvctl__subcmd__asus__subcmd__aura__subcmd__help,color)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__color"
                ;;
            nvctl__subcmd__asus__subcmd__aura__subcmd__help,help)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__asus__subcmd__aura__subcmd__help,mode)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__mode"
                ;;
            nvctl__subcmd__asus__subcmd__aura__subcmd__help,restore)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__restore"
                ;;
            nvctl__subcmd__asus__subcmd__aura__subcmd__help,status)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__asus__subcmd__aura__subcmd__help,temp-reactive)
                cmd="nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__temp__subcmd__reactive"
                ;;
            nvctl__subcmd__asus__subcmd__help,aura)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__aura"
                ;;
            nvctl__subcmd__asus__subcmd__help,detect)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__detect"
                ;;
            nvctl__subcmd__asus__subcmd__help,help)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__asus__subcmd__help,power)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__power"
                ;;
            nvctl__subcmd__asus__subcmd__help,status)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__asus__subcmd__help__subcmd__aura,color)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__color"
                ;;
            nvctl__subcmd__asus__subcmd__help__subcmd__aura,mode)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__mode"
                ;;
            nvctl__subcmd__asus__subcmd__help__subcmd__aura,restore)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__restore"
                ;;
            nvctl__subcmd__asus__subcmd__help__subcmd__aura,status)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__status"
                ;;
            nvctl__subcmd__asus__subcmd__help__subcmd__aura,temp-reactive)
                cmd="nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__temp__subcmd__reactive"
                ;;
            nvctl__subcmd__color,help)
                cmd="nvctl__subcmd__color__subcmd__help"
                ;;
            nvctl__subcmd__color,profiles)
                cmd="nvctl__subcmd__color__subcmd__profiles"
                ;;
            nvctl__subcmd__color,vibrance)
                cmd="nvctl__subcmd__color__subcmd__vibrance"
                ;;
            nvctl__subcmd__color__subcmd__help,help)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__color__subcmd__help,profiles)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__profiles"
                ;;
            nvctl__subcmd__color__subcmd__help,vibrance)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__vibrance"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__profiles,apply)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__apply"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__profiles,create)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__create"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__profiles,list)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__list"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__profiles,schedule)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__schedule"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__vibrance,apply)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__apply"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__vibrance,get)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__get"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__vibrance,preview)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__preview"
                ;;
            nvctl__subcmd__color__subcmd__help__subcmd__vibrance,set)
                cmd="nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__set"
                ;;
            nvctl__subcmd__color__subcmd__profiles,apply)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__apply"
                ;;
            nvctl__subcmd__color__subcmd__profiles,create)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__create"
                ;;
            nvctl__subcmd__color__subcmd__profiles,help)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__help"
                ;;
            nvctl__subcmd__color__subcmd__profiles,list)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__list"
                ;;
            nvctl__subcmd__color__subcmd__profiles,schedule)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__schedule"
                ;;
            nvctl__subcmd__color__subcmd__profiles__subcmd__help,apply)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__apply"
                ;;
            nvctl__subcmd__color__subcmd__profiles__subcmd__help,create)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__create"
                ;;
            nvctl__subcmd__color__subcmd__profiles__subcmd__help,help)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__color__subcmd__profiles__subcmd__help,list)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__color__subcmd__profiles__subcmd__help,schedule)
                cmd="nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__schedule"
                ;;
            nvctl__subcmd__color__subcmd__vibrance,apply)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__apply"
                ;;
            nvctl__subcmd__color__subcmd__vibrance,get)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__get"
                ;;
            nvctl__subcmd__color__subcmd__vibrance,help)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__help"
                ;;
            nvctl__subcmd__color__subcmd__vibrance,preview)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__preview"
                ;;
            nvctl__subcmd__color__subcmd__vibrance,set)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__set"
                ;;
            nvctl__subcmd__color__subcmd__vibrance__subcmd__help,apply)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__apply"
                ;;
            nvctl__subcmd__color__subcmd__vibrance__subcmd__help,get)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__color__subcmd__vibrance__subcmd__help,help)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__color__subcmd__vibrance__subcmd__help,preview)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__preview"
                ;;
            nvctl__subcmd__color__subcmd__vibrance__subcmd__help,set)
                cmd="nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__companion,help)
                cmd="nvctl__subcmd__companion__subcmd__help"
                ;;
            nvctl__subcmd__companion,notify-test)
                cmd="nvctl__subcmd__companion__subcmd__notify__subcmd__test"
                ;;
            nvctl__subcmd__companion,open-docs)
                cmd="nvctl__subcmd__companion__subcmd__open__subcmd__docs"
                ;;
            nvctl__subcmd__companion__subcmd__help,help)
                cmd="nvctl__subcmd__companion__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__companion__subcmd__help,notify-test)
                cmd="nvctl__subcmd__companion__subcmd__help__subcmd__notify__subcmd__test"
                ;;
            nvctl__subcmd__companion__subcmd__help,open-docs)
                cmd="nvctl__subcmd__companion__subcmd__help__subcmd__open__subcmd__docs"
                ;;
            nvctl__subcmd__config,apply)
                cmd="nvctl__subcmd__config__subcmd__apply"
                ;;
            nvctl__subcmd__config,backup)
                cmd="nvctl__subcmd__config__subcmd__backup"
                ;;
            nvctl__subcmd__config,capture)
                cmd="nvctl__subcmd__config__subcmd__capture"
                ;;
            nvctl__subcmd__config,diff)
                cmd="nvctl__subcmd__config__subcmd__diff"
                ;;
            nvctl__subcmd__config,edit)
                cmd="nvctl__subcmd__config__subcmd__edit"
                ;;
            nvctl__subcmd__config,export)
                cmd="nvctl__subcmd__config__subcmd__export"
                ;;
            nvctl__subcmd__config,help)
                cmd="nvctl__subcmd__config__subcmd__help"
                ;;
            nvctl__subcmd__config,import)
                cmd="nvctl__subcmd__config__subcmd__import"
                ;;
            nvctl__subcmd__config,preview)
                cmd="nvctl__subcmd__config__subcmd__preview"
                ;;
            nvctl__subcmd__config,profiles)
                cmd="nvctl__subcmd__config__subcmd__profiles"
                ;;
            nvctl__subcmd__config,reset)
                cmd="nvctl__subcmd__config__subcmd__reset"
                ;;
            nvctl__subcmd__config,restore)
                cmd="nvctl__subcmd__config__subcmd__restore"
                ;;
            nvctl__subcmd__config,show)
                cmd="nvctl__subcmd__config__subcmd__show"
                ;;
            nvctl__subcmd__config__subcmd__help,apply)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__apply"
                ;;
            nvctl__subcmd__config__subcmd__help,backup)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__backup"
                ;;
            nvctl__subcmd__config__subcmd__help,capture)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__capture"
                ;;
            nvctl__subcmd__config__subcmd__help,diff)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__diff"
                ;;
            nvctl__subcmd__config__subcmd__help,edit)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__edit"
                ;;
            nvctl__subcmd__config__subcmd__help,export)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__export"
                ;;
            nvctl__subcmd__config__subcmd__help,help)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__config__subcmd__help,import)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__import"
                ;;
            nvctl__subcmd__config__subcmd__help,preview)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__preview"
                ;;
            nvctl__subcmd__config__subcmd__help,profiles)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__profiles"
                ;;
            nvctl__subcmd__config__subcmd__help,reset)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__reset"
                ;;
            nvctl__subcmd__config__subcmd__help,restore)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__restore"
                ;;
            nvctl__subcmd__config__subcmd__help,show)
                cmd="nvctl__subcmd__config__subcmd__help__subcmd__show"
                ;;
            nvctl__subcmd__container,help)
                cmd="nvctl__subcmd__container__subcmd__help"
                ;;
            nvctl__subcmd__container,launch)
                cmd="nvctl__subcmd__container__subcmd__launch"
                ;;
            nvctl__subcmd__container,list)
                cmd="nvctl__subcmd__container__subcmd__list"
                ;;
            nvctl__subcmd__container,monitor)
                cmd="nvctl__subcmd__container__subcmd__monitor"
                ;;
            nvctl__subcmd__container,phantom-link)
                cmd="nvctl__subcmd__container__subcmd__phantom__subcmd__link"
                ;;
            nvctl__subcmd__container,profiles)
                cmd="nvctl__subcmd__container__subcmd__profiles"
                ;;
            nvctl__subcmd__container,runtime)
                cmd="nvctl__subcmd__container__subcmd__runtime"
                ;;
            nvctl__subcmd__container,status)
                cmd="nvctl__subcmd__container__subcmd__status"
                ;;
            nvctl__subcmd__container__subcmd__help,help)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__container__subcmd__help,launch)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__launch"
                ;;
            nvctl__subcmd__container__subcmd__help,list)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__container__subcmd__help,monitor)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__monitor"
                ;;
            nvctl__subcmd__container__subcmd__help,phantom-link)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__phantom__subcmd__link"
                ;;
            nvctl__subcmd__container__subcmd__help,profiles)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__profiles"
                ;;
            nvctl__subcmd__container__subcmd__help,runtime)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__runtime"
                ;;
            nvctl__subcmd__container__subcmd__help,status)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__profiles,apply)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__apply"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__profiles,create)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__create"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__profiles,list)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__list"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__runtime,configure)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__configure"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__runtime,doctor)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__doctor"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__runtime,info)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__info"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__runtime,setup)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__setup"
                ;;
            nvctl__subcmd__container__subcmd__help__subcmd__runtime,test)
                cmd="nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__test"
                ;;
            nvctl__subcmd__container__subcmd__profiles,apply)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__apply"
                ;;
            nvctl__subcmd__container__subcmd__profiles,create)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__create"
                ;;
            nvctl__subcmd__container__subcmd__profiles,help)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__help"
                ;;
            nvctl__subcmd__container__subcmd__profiles,list)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__list"
                ;;
            nvctl__subcmd__container__subcmd__profiles__subcmd__help,apply)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__apply"
                ;;
            nvctl__subcmd__container__subcmd__profiles__subcmd__help,create)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__create"
                ;;
            nvctl__subcmd__container__subcmd__profiles__subcmd__help,help)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__container__subcmd__profiles__subcmd__help,list)
                cmd="nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__container__subcmd__runtime,configure)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__configure"
                ;;
            nvctl__subcmd__container__subcmd__runtime,doctor)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__doctor"
                ;;
            nvctl__subcmd__container__subcmd__runtime,help)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help"
                ;;
            nvctl__subcmd__container__subcmd__runtime,info)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__info"
                ;;
            nvctl__subcmd__container__subcmd__runtime,setup)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__setup"
                ;;
            nvctl__subcmd__container__subcmd__runtime,test)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__test"
                ;;
            nvctl__subcmd__container__subcmd__runtime__subcmd__help,configure)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__configure"
                ;;
            nvctl__subcmd__container__subcmd__runtime__subcmd__help,doctor)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__doctor"
                ;;
            nvctl__subcmd__container__subcmd__runtime__subcmd__help,help)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__container__subcmd__runtime__subcmd__help,info)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__container__subcmd__runtime__subcmd__help,setup)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__setup"
                ;;
            nvctl__subcmd__container__subcmd__runtime__subcmd__help,test)
                cmd="nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__test"
                ;;
            nvctl__subcmd__display,color-range)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range"
                ;;
            nvctl__subcmd__display,color-space)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space"
                ;;
            nvctl__subcmd__display,dithering)
                cmd="nvctl__subcmd__display__subcmd__dithering"
                ;;
            nvctl__subcmd__display,gamma)
                cmd="nvctl__subcmd__display__subcmd__gamma"
                ;;
            nvctl__subcmd__display,hdr)
                cmd="nvctl__subcmd__display__subcmd__hdr"
                ;;
            nvctl__subcmd__display,help)
                cmd="nvctl__subcmd__display__subcmd__help"
                ;;
            nvctl__subcmd__display,info)
                cmd="nvctl__subcmd__display__subcmd__info"
                ;;
            nvctl__subcmd__display,ls)
                cmd="nvctl__subcmd__display__subcmd__ls"
                ;;
            nvctl__subcmd__display,sharpening)
                cmd="nvctl__subcmd__display__subcmd__sharpening"
                ;;
            nvctl__subcmd__display,vibrance)
                cmd="nvctl__subcmd__display__subcmd__vibrance"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__range,get)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__range,help)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__range,set)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help,get)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help,set)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__space,get)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__space,help)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__space,set)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help,get)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help,set)
                cmd="nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__dithering,disable)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__disable"
                ;;
            nvctl__subcmd__display__subcmd__dithering,enable)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__enable"
                ;;
            nvctl__subcmd__display__subcmd__dithering,get)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__dithering,help)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__dithering__subcmd__help,disable)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__display__subcmd__dithering__subcmd__help,enable)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__display__subcmd__dithering__subcmd__help,get)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__dithering__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__gamma,get)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__gamma,help)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__gamma,reset)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__gamma,set)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__gamma__subcmd__help,get)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__gamma__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__gamma__subcmd__help,reset)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__gamma__subcmd__help,set)
                cmd="nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__hdr,disable)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__disable"
                ;;
            nvctl__subcmd__display__subcmd__hdr,enable)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__enable"
                ;;
            nvctl__subcmd__display__subcmd__hdr,help)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__hdr,status)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__status"
                ;;
            nvctl__subcmd__display__subcmd__hdr,toggle)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__toggle"
                ;;
            nvctl__subcmd__display__subcmd__hdr__subcmd__help,disable)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__display__subcmd__hdr__subcmd__help,enable)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__display__subcmd__hdr__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__hdr__subcmd__help,status)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__display__subcmd__hdr__subcmd__help,toggle)
                cmd="nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__toggle"
                ;;
            nvctl__subcmd__display__subcmd__help,color-range)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range"
                ;;
            nvctl__subcmd__display__subcmd__help,color-space)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space"
                ;;
            nvctl__subcmd__display__subcmd__help,dithering)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__dithering"
                ;;
            nvctl__subcmd__display__subcmd__help,gamma)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__gamma"
                ;;
            nvctl__subcmd__display__subcmd__help,hdr)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__hdr"
                ;;
            nvctl__subcmd__display__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__help,info)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__help,ls)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__ls"
                ;;
            nvctl__subcmd__display__subcmd__help,sharpening)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__sharpening"
                ;;
            nvctl__subcmd__display__subcmd__help,vibrance)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range,get)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range,set)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space,get)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space,set)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__dithering,disable)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__disable"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__dithering,enable)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__enable"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__dithering,get)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__gamma,get)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__gamma,reset)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__gamma,set)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__hdr,disable)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__disable"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__hdr,enable)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__enable"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__hdr,status)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__status"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__hdr,toggle)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__toggle"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__sharpening,get)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__sharpening,info)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__sharpening,reset)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__sharpening,set)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,get)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,info)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,list)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__list"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,reset)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,set)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,set-display)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set__subcmd__display"
                ;;
            nvctl__subcmd__display__subcmd__help__subcmd__vibrance,set-raw)
                cmd="nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set__subcmd__raw"
                ;;
            nvctl__subcmd__display__subcmd__sharpening,get)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__sharpening,help)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__sharpening,info)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__sharpening,reset)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__sharpening,set)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__sharpening__subcmd__help,get)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__sharpening__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__sharpening__subcmd__help,info)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__sharpening__subcmd__help,reset)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__sharpening__subcmd__help,set)
                cmd="nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,get)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,help)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,info)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,list)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__list"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,reset)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,set)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,set-display)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__display"
                ;;
            nvctl__subcmd__display__subcmd__vibrance,set-raw)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__raw"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,get)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__get"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,help)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,info)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,list)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,reset)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__reset"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,set)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,set-display)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set__subcmd__display"
                ;;
            nvctl__subcmd__display__subcmd__vibrance__subcmd__help,set-raw)
                cmd="nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set__subcmd__raw"
                ;;
            nvctl__subcmd__dlss,auto)
                cmd="nvctl__subcmd__dlss__subcmd__auto"
                ;;
            nvctl__subcmd__dlss,disable)
                cmd="nvctl__subcmd__dlss__subcmd__disable"
                ;;
            nvctl__subcmd__dlss,doctor)
                cmd="nvctl__subcmd__dlss__subcmd__doctor"
                ;;
            nvctl__subcmd__dlss,enable)
                cmd="nvctl__subcmd__dlss__subcmd__enable"
                ;;
            nvctl__subcmd__dlss,games)
                cmd="nvctl__subcmd__dlss__subcmd__games"
                ;;
            nvctl__subcmd__dlss,help)
                cmd="nvctl__subcmd__dlss__subcmd__help"
                ;;
            nvctl__subcmd__dlss,info)
                cmd="nvctl__subcmd__dlss__subcmd__info"
                ;;
            nvctl__subcmd__dlss,launch-opts)
                cmd="nvctl__subcmd__dlss__subcmd__launch__subcmd__opts"
                ;;
            nvctl__subcmd__dlss,metrics)
                cmd="nvctl__subcmd__dlss__subcmd__metrics"
                ;;
            nvctl__subcmd__dlss,profiles)
                cmd="nvctl__subcmd__dlss__subcmd__profiles"
                ;;
            nvctl__subcmd__dlss,status)
                cmd="nvctl__subcmd__dlss__subcmd__status"
                ;;
            nvctl__subcmd__dlss,versions)
                cmd="nvctl__subcmd__dlss__subcmd__versions"
                ;;
            nvctl__subcmd__dlss__subcmd__help,auto)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__auto"
                ;;
            nvctl__subcmd__dlss__subcmd__help,disable)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__dlss__subcmd__help,doctor)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__doctor"
                ;;
            nvctl__subcmd__dlss__subcmd__help,enable)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__dlss__subcmd__help,games)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__games"
                ;;
            nvctl__subcmd__dlss__subcmd__help,help)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__dlss__subcmd__help,info)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__dlss__subcmd__help,launch-opts)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__launch__subcmd__opts"
                ;;
            nvctl__subcmd__dlss__subcmd__help,metrics)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__metrics"
                ;;
            nvctl__subcmd__dlss__subcmd__help,profiles)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__profiles"
                ;;
            nvctl__subcmd__dlss__subcmd__help,status)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__dlss__subcmd__help,versions)
                cmd="nvctl__subcmd__dlss__subcmd__help__subcmd__versions"
                ;;
            nvctl__subcmd__driver,capabilities)
                cmd="nvctl__subcmd__driver__subcmd__capabilities"
                ;;
            nvctl__subcmd__driver,check)
                cmd="nvctl__subcmd__driver__subcmd__check"
                ;;
            nvctl__subcmd__driver,diagnose-release)
                cmd="nvctl__subcmd__driver__subcmd__diagnose__subcmd__release"
                ;;
            nvctl__subcmd__driver,dkms)
                cmd="nvctl__subcmd__driver__subcmd__dkms"
                ;;
            nvctl__subcmd__driver,gsp)
                cmd="nvctl__subcmd__driver__subcmd__gsp"
                ;;
            nvctl__subcmd__driver,help)
                cmd="nvctl__subcmd__driver__subcmd__help"
                ;;
            nvctl__subcmd__driver,info)
                cmd="nvctl__subcmd__driver__subcmd__info"
                ;;
            nvctl__subcmd__driver,install)
                cmd="nvctl__subcmd__driver__subcmd__install"
                ;;
            nvctl__subcmd__driver,logs)
                cmd="nvctl__subcmd__driver__subcmd__logs"
                ;;
            nvctl__subcmd__driver,rollback)
                cmd="nvctl__subcmd__driver__subcmd__rollback"
                ;;
            nvctl__subcmd__driver,source)
                cmd="nvctl__subcmd__driver__subcmd__source"
                ;;
            nvctl__subcmd__driver,support-bundle)
                cmd="nvctl__subcmd__driver__subcmd__support__subcmd__bundle"
                ;;
            nvctl__subcmd__driver,update)
                cmd="nvctl__subcmd__driver__subcmd__update"
                ;;
            nvctl__subcmd__driver,validate)
                cmd="nvctl__subcmd__driver__subcmd__validate"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,build)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__build"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,cleanup)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__cleanup"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,doctor)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__doctor"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,fix)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__fix"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,help)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,hook)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__hook"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,logs)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__logs"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,setup)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__setup"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,status)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__dkms,unregister)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__unregister"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,build)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__build"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,cleanup)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__cleanup"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,doctor)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__doctor"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,fix)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__fix"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,help)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,hook)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__hook"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,logs)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__logs"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,setup)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__setup"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,status)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__dkms__subcmd__help,unregister)
                cmd="nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__unregister"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,check-update)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__check__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,diagnostics)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__diagnostics"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,disable)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__disable"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,enable)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__enable"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,explain)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__explain"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,help)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,status)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__gsp,update)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,check-update)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__check__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,diagnostics)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__diagnostics"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,disable)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,enable)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,explain)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__explain"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,help)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,status)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__gsp__subcmd__help,update)
                cmd="nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__help,capabilities)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__capabilities"
                ;;
            nvctl__subcmd__driver__subcmd__help,check)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__check"
                ;;
            nvctl__subcmd__driver__subcmd__help,diagnose-release)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__diagnose__subcmd__release"
                ;;
            nvctl__subcmd__driver__subcmd__help,dkms)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms"
                ;;
            nvctl__subcmd__driver__subcmd__help,gsp)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp"
                ;;
            nvctl__subcmd__driver__subcmd__help,help)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__help,info)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__driver__subcmd__help,install)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__install"
                ;;
            nvctl__subcmd__driver__subcmd__help,logs)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__logs"
                ;;
            nvctl__subcmd__driver__subcmd__help,rollback)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__rollback"
                ;;
            nvctl__subcmd__driver__subcmd__help,source)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__source"
                ;;
            nvctl__subcmd__driver__subcmd__help,support-bundle)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__support__subcmd__bundle"
                ;;
            nvctl__subcmd__driver__subcmd__help,update)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__help,validate)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__validate"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,build)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__build"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,cleanup)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__cleanup"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,doctor)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__doctor"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,fix)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__fix"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,hook)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__hook"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,logs)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__logs"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,setup)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__setup"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,status)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__dkms,unregister)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__unregister"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,check-update)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__check__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,diagnostics)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__diagnostics"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,disable)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__disable"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,enable)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__enable"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,explain)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__explain"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,status)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__gsp,update)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__source,doctor)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__doctor"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__source,init)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__init"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__source,status)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__source,sync)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__sync"
                ;;
            nvctl__subcmd__driver__subcmd__help__subcmd__source,update)
                cmd="nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__source,doctor)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__doctor"
                ;;
            nvctl__subcmd__driver__subcmd__source,help)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__source,init)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__init"
                ;;
            nvctl__subcmd__driver__subcmd__source,status)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__source,sync)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__sync"
                ;;
            nvctl__subcmd__driver__subcmd__source,update)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__update"
                ;;
            nvctl__subcmd__driver__subcmd__source__subcmd__help,doctor)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__doctor"
                ;;
            nvctl__subcmd__driver__subcmd__source__subcmd__help,help)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__driver__subcmd__source__subcmd__help,init)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__init"
                ;;
            nvctl__subcmd__driver__subcmd__source__subcmd__help,status)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__driver__subcmd__source__subcmd__help,sync)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__sync"
                ;;
            nvctl__subcmd__driver__subcmd__source__subcmd__help,update)
                cmd="nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__update"
                ;;
            nvctl__subcmd__fan,help)
                cmd="nvctl__subcmd__fan__subcmd__help"
                ;;
            nvctl__subcmd__fan,info)
                cmd="nvctl__subcmd__fan__subcmd__info"
                ;;
            nvctl__subcmd__fan,set)
                cmd="nvctl__subcmd__fan__subcmd__set"
                ;;
            nvctl__subcmd__fan__subcmd__help,help)
                cmd="nvctl__subcmd__fan__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__fan__subcmd__help,info)
                cmd="nvctl__subcmd__fan__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__fan__subcmd__help,set)
                cmd="nvctl__subcmd__fan__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__gaming,auto)
                cmd="nvctl__subcmd__gaming__subcmd__auto"
                ;;
            nvctl__subcmd__gaming,disable)
                cmd="nvctl__subcmd__gaming__subcmd__disable"
                ;;
            nvctl__subcmd__gaming,enable)
                cmd="nvctl__subcmd__gaming__subcmd__enable"
                ;;
            nvctl__subcmd__gaming,gamescope)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope"
                ;;
            nvctl__subcmd__gaming,help)
                cmd="nvctl__subcmd__gaming__subcmd__help"
                ;;
            nvctl__subcmd__gaming,latency)
                cmd="nvctl__subcmd__gaming__subcmd__latency"
                ;;
            nvctl__subcmd__gaming,launch)
                cmd="nvctl__subcmd__gaming__subcmd__launch"
                ;;
            nvctl__subcmd__gaming,status)
                cmd="nvctl__subcmd__gaming__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,config)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__config"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,daemon)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__daemon"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,disable)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__disable"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,disable-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__disable__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,enable)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__enable"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,enable-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__enable__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,help)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,install-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__install__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,start)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__start"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,status)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,stop)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__stop"
                ;;
            nvctl__subcmd__gaming__subcmd__auto,uninstall-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__uninstall__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,config)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__config"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,daemon)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__daemon"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,disable)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,disable-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,enable)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,enable-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,help)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,install-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__install__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,start)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__start"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,status)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,stop)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__stop"
                ;;
            nvctl__subcmd__gaming__subcmd__auto__subcmd__help,uninstall-service)
                cmd="nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__uninstall__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope,create-preset)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__create__subcmd__preset"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope,help)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope,launch)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__launch"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope,presets)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__presets"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help,create-preset)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__create__subcmd__preset"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help,help)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help,launch)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__launch"
                ;;
            nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help,presets)
                cmd="nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__presets"
                ;;
            nvctl__subcmd__gaming__subcmd__help,auto)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto"
                ;;
            nvctl__subcmd__gaming__subcmd__help,disable)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__gaming__subcmd__help,enable)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__gaming__subcmd__help,gamescope)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope"
                ;;
            nvctl__subcmd__gaming__subcmd__help,help)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__help,latency)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__latency"
                ;;
            nvctl__subcmd__gaming__subcmd__help,launch)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch"
                ;;
            nvctl__subcmd__gaming__subcmd__help,status)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,config)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__config"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,daemon)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__daemon"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,disable)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,disable-service)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,enable)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,enable-service)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,install-service)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__install__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,start)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__start"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,status)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,stop)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__stop"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__auto,uninstall-service)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__uninstall__subcmd__service"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope,create-preset)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__create__subcmd__preset"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope,launch)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__launch"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope,presets)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__presets"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__latency,optimize)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__optimize"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__latency,status)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__latency,test)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__test"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,create)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__create"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,delete)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__delete"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,examples)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__examples"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,hook-add)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook__subcmd__add"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,hook-list)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook__subcmd__list"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,hook-remove)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook__subcmd__remove"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,list)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__list"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,run)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__run"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,set-gamescope-preset)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__set__subcmd__gamescope__subcmd__preset"
                ;;
            nvctl__subcmd__gaming__subcmd__help__subcmd__launch,show)
                cmd="nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__show"
                ;;
            nvctl__subcmd__gaming__subcmd__latency,help)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__latency,optimize)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__optimize"
                ;;
            nvctl__subcmd__gaming__subcmd__latency,status)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__latency,test)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__test"
                ;;
            nvctl__subcmd__gaming__subcmd__latency__subcmd__help,help)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__latency__subcmd__help,optimize)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__optimize"
                ;;
            nvctl__subcmd__gaming__subcmd__latency__subcmd__help,status)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__gaming__subcmd__latency__subcmd__help,test)
                cmd="nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__test"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,create)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__create"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,delete)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__delete"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,examples)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__examples"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,help)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,hook-add)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__add"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,hook-list)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__list"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,hook-remove)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__remove"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,list)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__list"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,run)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__run"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,set-gamescope-preset)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__set__subcmd__gamescope__subcmd__preset"
                ;;
            nvctl__subcmd__gaming__subcmd__launch,show)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__show"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,create)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__create"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,delete)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__delete"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,examples)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__examples"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,help)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,hook-add)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook__subcmd__add"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,hook-list)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook__subcmd__list"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,hook-remove)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook__subcmd__remove"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,list)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,run)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__run"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,set-gamescope-preset)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__set__subcmd__gamescope__subcmd__preset"
                ;;
            nvctl__subcmd__gaming__subcmd__launch__subcmd__help,show)
                cmd="nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__show"
                ;;
            nvctl__subcmd__gpu,benchmark)
                cmd="nvctl__subcmd__gpu__subcmd__benchmark"
                ;;
            nvctl__subcmd__gpu,capabilities)
                cmd="nvctl__subcmd__gpu__subcmd__capabilities"
                ;;
            nvctl__subcmd__gpu,export)
                cmd="nvctl__subcmd__gpu__subcmd__export"
                ;;
            nvctl__subcmd__gpu,help)
                cmd="nvctl__subcmd__gpu__subcmd__help"
                ;;
            nvctl__subcmd__gpu,info)
                cmd="nvctl__subcmd__gpu__subcmd__info"
                ;;
            nvctl__subcmd__gpu,list)
                cmd="nvctl__subcmd__gpu__subcmd__list"
                ;;
            nvctl__subcmd__gpu,select)
                cmd="nvctl__subcmd__gpu__subcmd__select"
                ;;
            nvctl__subcmd__gpu,stat)
                cmd="nvctl__subcmd__gpu__subcmd__stat"
                ;;
            nvctl__subcmd__gpu,stress)
                cmd="nvctl__subcmd__gpu__subcmd__stress"
                ;;
            nvctl__subcmd__gpu,watch)
                cmd="nvctl__subcmd__gpu__subcmd__watch"
                ;;
            nvctl__subcmd__gpu__subcmd__help,benchmark)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__benchmark"
                ;;
            nvctl__subcmd__gpu__subcmd__help,capabilities)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__capabilities"
                ;;
            nvctl__subcmd__gpu__subcmd__help,export)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__export"
                ;;
            nvctl__subcmd__gpu__subcmd__help,help)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__gpu__subcmd__help,info)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__gpu__subcmd__help,list)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__gpu__subcmd__help,select)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__select"
                ;;
            nvctl__subcmd__gpu__subcmd__help,stat)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__stat"
                ;;
            nvctl__subcmd__gpu__subcmd__help,stress)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__stress"
                ;;
            nvctl__subcmd__gpu__subcmd__help,watch)
                cmd="nvctl__subcmd__gpu__subcmd__help__subcmd__watch"
                ;;
            nvctl__subcmd__hdr,capabilities)
                cmd="nvctl__subcmd__hdr__subcmd__capabilities"
                ;;
            nvctl__subcmd__hdr,config)
                cmd="nvctl__subcmd__hdr__subcmd__config"
                ;;
            nvctl__subcmd__hdr,disable)
                cmd="nvctl__subcmd__hdr__subcmd__disable"
                ;;
            nvctl__subcmd__hdr,enable)
                cmd="nvctl__subcmd__hdr__subcmd__enable"
                ;;
            nvctl__subcmd__hdr,help)
                cmd="nvctl__subcmd__hdr__subcmd__help"
                ;;
            nvctl__subcmd__hdr,set-brightness)
                cmd="nvctl__subcmd__hdr__subcmd__set__subcmd__brightness"
                ;;
            nvctl__subcmd__hdr,status)
                cmd="nvctl__subcmd__hdr__subcmd__status"
                ;;
            nvctl__subcmd__hdr,tools)
                cmd="nvctl__subcmd__hdr__subcmd__tools"
                ;;
            nvctl__subcmd__hdr__subcmd__help,capabilities)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__capabilities"
                ;;
            nvctl__subcmd__hdr__subcmd__help,config)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__config"
                ;;
            nvctl__subcmd__hdr__subcmd__help,disable)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__hdr__subcmd__help,enable)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__hdr__subcmd__help,help)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__hdr__subcmd__help,set-brightness)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__set__subcmd__brightness"
                ;;
            nvctl__subcmd__hdr__subcmd__help,status)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__hdr__subcmd__help,tools)
                cmd="nvctl__subcmd__hdr__subcmd__help__subcmd__tools"
                ;;
            nvctl__subcmd__help,arch)
                cmd="nvctl__subcmd__help__subcmd__arch"
                ;;
            nvctl__subcmd__help,asus)
                cmd="nvctl__subcmd__help__subcmd__asus"
                ;;
            nvctl__subcmd__help,color)
                cmd="nvctl__subcmd__help__subcmd__color"
                ;;
            nvctl__subcmd__help,companion)
                cmd="nvctl__subcmd__help__subcmd__companion"
                ;;
            nvctl__subcmd__help,completion)
                cmd="nvctl__subcmd__help__subcmd__completion"
                ;;
            nvctl__subcmd__help,config)
                cmd="nvctl__subcmd__help__subcmd__config"
                ;;
            nvctl__subcmd__help,container)
                cmd="nvctl__subcmd__help__subcmd__container"
                ;;
            nvctl__subcmd__help,display)
                cmd="nvctl__subcmd__help__subcmd__display"
                ;;
            nvctl__subcmd__help,dlss)
                cmd="nvctl__subcmd__help__subcmd__dlss"
                ;;
            nvctl__subcmd__help,doctor)
                cmd="nvctl__subcmd__help__subcmd__doctor"
                ;;
            nvctl__subcmd__help,driver)
                cmd="nvctl__subcmd__help__subcmd__driver"
                ;;
            nvctl__subcmd__help,fan)
                cmd="nvctl__subcmd__help__subcmd__fan"
                ;;
            nvctl__subcmd__help,gaming)
                cmd="nvctl__subcmd__help__subcmd__gaming"
                ;;
            nvctl__subcmd__help,gpu)
                cmd="nvctl__subcmd__help__subcmd__gpu"
                ;;
            nvctl__subcmd__help,hdr)
                cmd="nvctl__subcmd__help__subcmd__hdr"
                ;;
            nvctl__subcmd__help,help)
                cmd="nvctl__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__help,interactive)
                cmd="nvctl__subcmd__help__subcmd__interactive"
                ;;
            nvctl__subcmd__help,kde)
                cmd="nvctl__subcmd__help__subcmd__kde"
                ;;
            nvctl__subcmd__help,monitor)
                cmd="nvctl__subcmd__help__subcmd__monitor"
                ;;
            nvctl__subcmd__help,monitors)
                cmd="nvctl__subcmd__help__subcmd__monitors"
                ;;
            nvctl__subcmd__help,nvtop)
                cmd="nvctl__subcmd__help__subcmd__nvtop"
                ;;
            nvctl__subcmd__help,osd)
                cmd="nvctl__subcmd__help__subcmd__osd"
                ;;
            nvctl__subcmd__help,overclock)
                cmd="nvctl__subcmd__help__subcmd__overclock"
                ;;
            nvctl__subcmd__help,passthrough)
                cmd="nvctl__subcmd__help__subcmd__passthrough"
                ;;
            nvctl__subcmd__help,power)
                cmd="nvctl__subcmd__help__subcmd__power"
                ;;
            nvctl__subcmd__help,power-profile)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile"
                ;;
            nvctl__subcmd__help,recording)
                cmd="nvctl__subcmd__help__subcmd__recording"
                ;;
            nvctl__subcmd__help,shaders)
                cmd="nvctl__subcmd__help__subcmd__shaders"
                ;;
            nvctl__subcmd__help,system)
                cmd="nvctl__subcmd__help__subcmd__system"
                ;;
            nvctl__subcmd__help,tui)
                cmd="nvctl__subcmd__help__subcmd__tui"
                ;;
            nvctl__subcmd__help,upscaling)
                cmd="nvctl__subcmd__help__subcmd__upscaling"
                ;;
            nvctl__subcmd__help,version)
                cmd="nvctl__subcmd__help__subcmd__version"
                ;;
            nvctl__subcmd__help,vibrance)
                cmd="nvctl__subcmd__help__subcmd__vibrance"
                ;;
            nvctl__subcmd__help,vrr)
                cmd="nvctl__subcmd__help__subcmd__vrr"
                ;;
            nvctl__subcmd__help,wayland)
                cmd="nvctl__subcmd__help__subcmd__wayland"
                ;;
            nvctl__subcmd__help__subcmd__arch,aur-suggestions)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__aur__subcmd__suggestions"
                ;;
            nvctl__subcmd__help__subcmd__arch,check-updates)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__check__subcmd__updates"
                ;;
            nvctl__subcmd__help__subcmd__arch,install-hooks)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__install__subcmd__hooks"
                ;;
            nvctl__subcmd__help__subcmd__arch,mkinitcpio)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__mkinitcpio"
                ;;
            nvctl__subcmd__help__subcmd__arch,rebuild-dkms)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__rebuild__subcmd__dkms"
                ;;
            nvctl__subcmd__help__subcmd__arch,remove-hooks)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__remove__subcmd__hooks"
                ;;
            nvctl__subcmd__help__subcmd__arch,status)
                cmd="nvctl__subcmd__help__subcmd__arch__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__asus,aura)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__aura"
                ;;
            nvctl__subcmd__help__subcmd__asus,detect)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__detect"
                ;;
            nvctl__subcmd__help__subcmd__asus,power)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__power"
                ;;
            nvctl__subcmd__help__subcmd__asus,status)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__asus__subcmd__aura,color)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__color"
                ;;
            nvctl__subcmd__help__subcmd__asus__subcmd__aura,mode)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__mode"
                ;;
            nvctl__subcmd__help__subcmd__asus__subcmd__aura,restore)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__restore"
                ;;
            nvctl__subcmd__help__subcmd__asus__subcmd__aura,status)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__asus__subcmd__aura,temp-reactive)
                cmd="nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__temp__subcmd__reactive"
                ;;
            nvctl__subcmd__help__subcmd__color,profiles)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__profiles"
                ;;
            nvctl__subcmd__help__subcmd__color,vibrance)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__vibrance"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__profiles,apply)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__apply"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__profiles,create)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__create"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__profiles,list)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__profiles,schedule)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__schedule"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__vibrance,apply)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__apply"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__vibrance,get)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__vibrance,preview)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__preview"
                ;;
            nvctl__subcmd__help__subcmd__color__subcmd__vibrance,set)
                cmd="nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__companion,notify-test)
                cmd="nvctl__subcmd__help__subcmd__companion__subcmd__notify__subcmd__test"
                ;;
            nvctl__subcmd__help__subcmd__companion,open-docs)
                cmd="nvctl__subcmd__help__subcmd__companion__subcmd__open__subcmd__docs"
                ;;
            nvctl__subcmd__help__subcmd__config,apply)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__apply"
                ;;
            nvctl__subcmd__help__subcmd__config,backup)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__backup"
                ;;
            nvctl__subcmd__help__subcmd__config,capture)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__capture"
                ;;
            nvctl__subcmd__help__subcmd__config,diff)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__diff"
                ;;
            nvctl__subcmd__help__subcmd__config,edit)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__edit"
                ;;
            nvctl__subcmd__help__subcmd__config,export)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__export"
                ;;
            nvctl__subcmd__help__subcmd__config,import)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__import"
                ;;
            nvctl__subcmd__help__subcmd__config,preview)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__preview"
                ;;
            nvctl__subcmd__help__subcmd__config,profiles)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__profiles"
                ;;
            nvctl__subcmd__help__subcmd__config,reset)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__reset"
                ;;
            nvctl__subcmd__help__subcmd__config,restore)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__restore"
                ;;
            nvctl__subcmd__help__subcmd__config,show)
                cmd="nvctl__subcmd__help__subcmd__config__subcmd__show"
                ;;
            nvctl__subcmd__help__subcmd__container,launch)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__launch"
                ;;
            nvctl__subcmd__help__subcmd__container,list)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__container,monitor)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__monitor"
                ;;
            nvctl__subcmd__help__subcmd__container,phantom-link)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__phantom__subcmd__link"
                ;;
            nvctl__subcmd__help__subcmd__container,profiles)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__profiles"
                ;;
            nvctl__subcmd__help__subcmd__container,runtime)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__runtime"
                ;;
            nvctl__subcmd__help__subcmd__container,status)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__profiles,apply)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__apply"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__profiles,create)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__create"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__profiles,list)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__runtime,configure)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__configure"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__runtime,doctor)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__doctor"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__runtime,info)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__runtime,setup)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__setup"
                ;;
            nvctl__subcmd__help__subcmd__container__subcmd__runtime,test)
                cmd="nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__test"
                ;;
            nvctl__subcmd__help__subcmd__display,color-range)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range"
                ;;
            nvctl__subcmd__help__subcmd__display,color-space)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space"
                ;;
            nvctl__subcmd__help__subcmd__display,dithering)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__dithering"
                ;;
            nvctl__subcmd__help__subcmd__display,gamma)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__gamma"
                ;;
            nvctl__subcmd__help__subcmd__display,hdr)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__hdr"
                ;;
            nvctl__subcmd__help__subcmd__display,info)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__display,ls)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__ls"
                ;;
            nvctl__subcmd__help__subcmd__display,sharpening)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__sharpening"
                ;;
            nvctl__subcmd__help__subcmd__display,vibrance)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range,get)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range,set)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space,get)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space,set)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__dithering,disable)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__dithering,enable)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__dithering,get)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__gamma,get)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__gamma,reset)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__reset"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__gamma,set)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__hdr,disable)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__hdr,enable)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__hdr,status)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__hdr,toggle)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__toggle"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__sharpening,get)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__sharpening,info)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__sharpening,reset)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__reset"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__sharpening,set)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,get)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__get"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,info)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,list)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,reset)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__reset"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,set)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,set-display)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__display"
                ;;
            nvctl__subcmd__help__subcmd__display__subcmd__vibrance,set-raw)
                cmd="nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__raw"
                ;;
            nvctl__subcmd__help__subcmd__dlss,auto)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__auto"
                ;;
            nvctl__subcmd__help__subcmd__dlss,disable)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__dlss,doctor)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__doctor"
                ;;
            nvctl__subcmd__help__subcmd__dlss,enable)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__dlss,games)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__games"
                ;;
            nvctl__subcmd__help__subcmd__dlss,info)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__dlss,launch-opts)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__launch__subcmd__opts"
                ;;
            nvctl__subcmd__help__subcmd__dlss,metrics)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__metrics"
                ;;
            nvctl__subcmd__help__subcmd__dlss,profiles)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__profiles"
                ;;
            nvctl__subcmd__help__subcmd__dlss,status)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__dlss,versions)
                cmd="nvctl__subcmd__help__subcmd__dlss__subcmd__versions"
                ;;
            nvctl__subcmd__help__subcmd__driver,capabilities)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__capabilities"
                ;;
            nvctl__subcmd__help__subcmd__driver,check)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__check"
                ;;
            nvctl__subcmd__help__subcmd__driver,diagnose-release)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__diagnose__subcmd__release"
                ;;
            nvctl__subcmd__help__subcmd__driver,dkms)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms"
                ;;
            nvctl__subcmd__help__subcmd__driver,gsp)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp"
                ;;
            nvctl__subcmd__help__subcmd__driver,info)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__driver,install)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__install"
                ;;
            nvctl__subcmd__help__subcmd__driver,logs)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__logs"
                ;;
            nvctl__subcmd__help__subcmd__driver,rollback)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__rollback"
                ;;
            nvctl__subcmd__help__subcmd__driver,source)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__source"
                ;;
            nvctl__subcmd__help__subcmd__driver,support-bundle)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__support__subcmd__bundle"
                ;;
            nvctl__subcmd__help__subcmd__driver,update)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__update"
                ;;
            nvctl__subcmd__help__subcmd__driver,validate)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__validate"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,build)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__build"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,cleanup)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__cleanup"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,doctor)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__doctor"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,fix)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__fix"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,hook)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__hook"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,logs)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__logs"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,setup)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__setup"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,status)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__dkms,unregister)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__unregister"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,check-update)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__check__subcmd__update"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,diagnostics)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__diagnostics"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,disable)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,enable)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,explain)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__explain"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,status)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__gsp,update)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__update"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__source,doctor)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__doctor"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__source,init)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__init"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__source,status)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__source,sync)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__sync"
                ;;
            nvctl__subcmd__help__subcmd__driver__subcmd__source,update)
                cmd="nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__update"
                ;;
            nvctl__subcmd__help__subcmd__fan,info)
                cmd="nvctl__subcmd__help__subcmd__fan__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__fan,set)
                cmd="nvctl__subcmd__help__subcmd__fan__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__gaming,auto)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto"
                ;;
            nvctl__subcmd__help__subcmd__gaming,disable)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__gaming,enable)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__gaming,gamescope)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope"
                ;;
            nvctl__subcmd__help__subcmd__gaming,latency)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__latency"
                ;;
            nvctl__subcmd__help__subcmd__gaming,launch)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch"
                ;;
            nvctl__subcmd__help__subcmd__gaming,status)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,config)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__config"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,daemon)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__daemon"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,disable)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,disable-service)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable__subcmd__service"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,enable)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,enable-service)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable__subcmd__service"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,install-service)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__install__subcmd__service"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,start)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__start"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,status)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,stop)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__stop"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__auto,uninstall-service)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__uninstall__subcmd__service"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope,create-preset)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__create__subcmd__preset"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope,launch)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__launch"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope,presets)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__presets"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__latency,optimize)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__optimize"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__latency,status)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__latency,test)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__test"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,create)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__create"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,delete)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__delete"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,examples)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__examples"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,hook-add)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__add"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,hook-list)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,hook-remove)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__remove"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,list)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,run)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__run"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,set-gamescope-preset)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__set__subcmd__gamescope__subcmd__preset"
                ;;
            nvctl__subcmd__help__subcmd__gaming__subcmd__launch,show)
                cmd="nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__show"
                ;;
            nvctl__subcmd__help__subcmd__gpu,benchmark)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__benchmark"
                ;;
            nvctl__subcmd__help__subcmd__gpu,capabilities)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__capabilities"
                ;;
            nvctl__subcmd__help__subcmd__gpu,export)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__export"
                ;;
            nvctl__subcmd__help__subcmd__gpu,info)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__gpu,list)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__gpu,select)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__select"
                ;;
            nvctl__subcmd__help__subcmd__gpu,stat)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__stat"
                ;;
            nvctl__subcmd__help__subcmd__gpu,stress)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__stress"
                ;;
            nvctl__subcmd__help__subcmd__gpu,watch)
                cmd="nvctl__subcmd__help__subcmd__gpu__subcmd__watch"
                ;;
            nvctl__subcmd__help__subcmd__hdr,capabilities)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__capabilities"
                ;;
            nvctl__subcmd__help__subcmd__hdr,config)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__config"
                ;;
            nvctl__subcmd__help__subcmd__hdr,disable)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__hdr,enable)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__hdr,set-brightness)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__set__subcmd__brightness"
                ;;
            nvctl__subcmd__help__subcmd__hdr,status)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__hdr,tools)
                cmd="nvctl__subcmd__help__subcmd__hdr__subcmd__tools"
                ;;
            nvctl__subcmd__help__subcmd__kde,gaming)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__gaming"
                ;;
            nvctl__subcmd__help__subcmd__kde,power-save)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__power__subcmd__save"
                ;;
            nvctl__subcmd__help__subcmd__kde,productivity)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__productivity"
                ;;
            nvctl__subcmd__help__subcmd__kde,restart)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__restart"
                ;;
            nvctl__subcmd__help__subcmd__kde,set-vrr)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__set__subcmd__vrr"
                ;;
            nvctl__subcmd__help__subcmd__kde,setup-env)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__setup__subcmd__env"
                ;;
            nvctl__subcmd__help__subcmd__kde,status)
                cmd="nvctl__subcmd__help__subcmd__kde__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__monitor,export)
                cmd="nvctl__subcmd__help__subcmd__monitor__subcmd__export"
                ;;
            nvctl__subcmd__help__subcmd__monitor,start)
                cmd="nvctl__subcmd__help__subcmd__monitor__subcmd__start"
                ;;
            nvctl__subcmd__help__subcmd__monitor,status)
                cmd="nvctl__subcmd__help__subcmd__monitor__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__monitor,stop)
                cmd="nvctl__subcmd__help__subcmd__monitor__subcmd__stop"
                ;;
            nvctl__subcmd__help__subcmd__monitor,tui)
                cmd="nvctl__subcmd__help__subcmd__monitor__subcmd__tui"
                ;;
            nvctl__subcmd__help__subcmd__monitors,apply-preset)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__apply__subcmd__preset"
                ;;
            nvctl__subcmd__help__subcmd__monitors,auto)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__auto"
                ;;
            nvctl__subcmd__help__subcmd__monitors,create-examples)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__create__subcmd__examples"
                ;;
            nvctl__subcmd__help__subcmd__monitors,gamescope)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__gamescope"
                ;;
            nvctl__subcmd__help__subcmd__monitors,list)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__monitors,load)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__load"
                ;;
            nvctl__subcmd__help__subcmd__monitors,presets)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__presets"
                ;;
            nvctl__subcmd__help__subcmd__monitors,preview)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__preview"
                ;;
            nvctl__subcmd__help__subcmd__monitors,save)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__save"
                ;;
            nvctl__subcmd__help__subcmd__monitors,set-vrr)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__set__subcmd__vrr"
                ;;
            nvctl__subcmd__help__subcmd__monitors,status)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__monitors,suggest)
                cmd="nvctl__subcmd__help__subcmd__monitors__subcmd__suggest"
                ;;
            nvctl__subcmd__help__subcmd__osd,add)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__add"
                ;;
            nvctl__subcmd__help__subcmd__osd,check)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__check"
                ;;
            nvctl__subcmd__help__subcmd__osd,config)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__config"
                ;;
            nvctl__subcmd__help__subcmd__osd,disable)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__osd,enable)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__osd,metrics)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__metrics"
                ;;
            nvctl__subcmd__help__subcmd__osd,remove)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__remove"
                ;;
            nvctl__subcmd__help__subcmd__osd,status)
                cmd="nvctl__subcmd__help__subcmd__osd__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__overclock,apply)
                cmd="nvctl__subcmd__help__subcmd__overclock__subcmd__apply"
                ;;
            nvctl__subcmd__help__subcmd__overclock,auto)
                cmd="nvctl__subcmd__help__subcmd__overclock__subcmd__auto"
                ;;
            nvctl__subcmd__help__subcmd__overclock,info)
                cmd="nvctl__subcmd__help__subcmd__overclock__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__overclock,profile)
                cmd="nvctl__subcmd__help__subcmd__overclock__subcmd__profile"
                ;;
            nvctl__subcmd__help__subcmd__overclock,reset)
                cmd="nvctl__subcmd__help__subcmd__overclock__subcmd__reset"
                ;;
            nvctl__subcmd__help__subcmd__overclock,stress-test)
                cmd="nvctl__subcmd__help__subcmd__overclock__subcmd__stress__subcmd__test"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,bind-vfio)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__bind__subcmd__vfio"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,hugepages)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__hugepages"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,iommu)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__iommu"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,list)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,persistent)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__persistent"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,qemu-command)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__qemu__subcmd__command"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,status)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,test-container)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__test__subcmd__container"
                ;;
            nvctl__subcmd__help__subcmd__passthrough,unbind-vfio)
                cmd="nvctl__subcmd__help__subcmd__passthrough__subcmd__unbind__subcmd__vfio"
                ;;
            nvctl__subcmd__help__subcmd__power,automate)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__automate"
                ;;
            nvctl__subcmd__help__subcmd__power,curve)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve"
                ;;
            nvctl__subcmd__help__subcmd__power,limit)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__limit"
                ;;
            nvctl__subcmd__help__subcmd__power,monitor)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__monitor"
                ;;
            nvctl__subcmd__help__subcmd__power,persistence)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__persistence"
                ;;
            nvctl__subcmd__help__subcmd__power,profile)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile"
                ;;
            nvctl__subcmd__help__subcmd__power,schedule)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__schedule"
                ;;
            nvctl__subcmd__help__subcmd__power,status)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,add)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__add"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,disable)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,edit)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__edit"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,enable)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,remove)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__remove"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,reset)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__reset"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__curve,show)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__show"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,apply)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__apply"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,auto-power)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__auto__subcmd__power"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,create-activity)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__create__subcmd__activity"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,create-defaults)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__create__subcmd__defaults"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,idle)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__idle"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,monitor)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__monitor"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,set)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__set"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__profile,status)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__schedule,add)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__add"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__schedule,disable)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__schedule,enable)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__schedule,list)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__list"
                ;;
            nvctl__subcmd__help__subcmd__power__subcmd__schedule,remove)
                cmd="nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__remove"
                ;;
            nvctl__subcmd__help__subcmd__recording,instant-replay)
                cmd="nvctl__subcmd__help__subcmd__recording__subcmd__instant__subcmd__replay"
                ;;
            nvctl__subcmd__help__subcmd__recording,presets)
                cmd="nvctl__subcmd__help__subcmd__recording__subcmd__presets"
                ;;
            nvctl__subcmd__help__subcmd__recording,save)
                cmd="nvctl__subcmd__help__subcmd__recording__subcmd__save"
                ;;
            nvctl__subcmd__help__subcmd__recording,start)
                cmd="nvctl__subcmd__help__subcmd__recording__subcmd__start"
                ;;
            nvctl__subcmd__help__subcmd__recording,status)
                cmd="nvctl__subcmd__help__subcmd__recording__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__recording,stop)
                cmd="nvctl__subcmd__help__subcmd__recording__subcmd__stop"
                ;;
            nvctl__subcmd__help__subcmd__shaders,clear)
                cmd="nvctl__subcmd__help__subcmd__shaders__subcmd__clear"
                ;;
            nvctl__subcmd__help__subcmd__shaders,open)
                cmd="nvctl__subcmd__help__subcmd__shaders__subcmd__open"
                ;;
            nvctl__subcmd__help__subcmd__shaders,optimize)
                cmd="nvctl__subcmd__help__subcmd__shaders__subcmd__optimize"
                ;;
            nvctl__subcmd__help__subcmd__shaders,precompile)
                cmd="nvctl__subcmd__help__subcmd__shaders__subcmd__precompile"
                ;;
            nvctl__subcmd__help__subcmd__shaders,stats)
                cmd="nvctl__subcmd__help__subcmd__shaders__subcmd__stats"
                ;;
            nvctl__subcmd__help__subcmd__system,compositor)
                cmd="nvctl__subcmd__help__subcmd__system__subcmd__compositor"
                ;;
            nvctl__subcmd__help__subcmd__system,distro)
                cmd="nvctl__subcmd__help__subcmd__system__subcmd__distro"
                ;;
            nvctl__subcmd__help__subcmd__system,info)
                cmd="nvctl__subcmd__help__subcmd__system__subcmd__info"
                ;;
            nvctl__subcmd__help__subcmd__system,optimize)
                cmd="nvctl__subcmd__help__subcmd__system__subcmd__optimize"
                ;;
            nvctl__subcmd__help__subcmd__upscaling,auto-detect)
                cmd="nvctl__subcmd__help__subcmd__upscaling__subcmd__auto__subcmd__detect"
                ;;
            nvctl__subcmd__help__subcmd__upscaling,disable)
                cmd="nvctl__subcmd__help__subcmd__upscaling__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__upscaling,enable)
                cmd="nvctl__subcmd__help__subcmd__upscaling__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__upscaling,profiles)
                cmd="nvctl__subcmd__help__subcmd__upscaling__subcmd__profiles"
                ;;
            nvctl__subcmd__help__subcmd__upscaling,status)
                cmd="nvctl__subcmd__help__subcmd__upscaling__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__vrr,configure)
                cmd="nvctl__subcmd__help__subcmd__vrr__subcmd__configure"
                ;;
            nvctl__subcmd__help__subcmd__vrr,disable)
                cmd="nvctl__subcmd__help__subcmd__vrr__subcmd__disable"
                ;;
            nvctl__subcmd__help__subcmd__vrr,enable)
                cmd="nvctl__subcmd__help__subcmd__vrr__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__vrr,status)
                cmd="nvctl__subcmd__help__subcmd__vrr__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__wayland,explicit-sync)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync"
                ;;
            nvctl__subcmd__help__subcmd__wayland,export-env)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__export__subcmd__env"
                ;;
            nvctl__subcmd__help__subcmd__wayland,optimize)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__optimize"
                ;;
            nvctl__subcmd__help__subcmd__wayland,status)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__status"
                ;;
            nvctl__subcmd__help__subcmd__wayland,switch-driver)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__switch__subcmd__driver"
                ;;
            nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync,enable)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__enable"
                ;;
            nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync,status)
                cmd="nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__status"
                ;;
            nvctl__subcmd__kde,gaming)
                cmd="nvctl__subcmd__kde__subcmd__gaming"
                ;;
            nvctl__subcmd__kde,help)
                cmd="nvctl__subcmd__kde__subcmd__help"
                ;;
            nvctl__subcmd__kde,power-save)
                cmd="nvctl__subcmd__kde__subcmd__power__subcmd__save"
                ;;
            nvctl__subcmd__kde,productivity)
                cmd="nvctl__subcmd__kde__subcmd__productivity"
                ;;
            nvctl__subcmd__kde,restart)
                cmd="nvctl__subcmd__kde__subcmd__restart"
                ;;
            nvctl__subcmd__kde,set-vrr)
                cmd="nvctl__subcmd__kde__subcmd__set__subcmd__vrr"
                ;;
            nvctl__subcmd__kde,setup-env)
                cmd="nvctl__subcmd__kde__subcmd__setup__subcmd__env"
                ;;
            nvctl__subcmd__kde,status)
                cmd="nvctl__subcmd__kde__subcmd__status"
                ;;
            nvctl__subcmd__kde__subcmd__help,gaming)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__gaming"
                ;;
            nvctl__subcmd__kde__subcmd__help,help)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__kde__subcmd__help,power-save)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__power__subcmd__save"
                ;;
            nvctl__subcmd__kde__subcmd__help,productivity)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__productivity"
                ;;
            nvctl__subcmd__kde__subcmd__help,restart)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__restart"
                ;;
            nvctl__subcmd__kde__subcmd__help,set-vrr)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__set__subcmd__vrr"
                ;;
            nvctl__subcmd__kde__subcmd__help,setup-env)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__setup__subcmd__env"
                ;;
            nvctl__subcmd__kde__subcmd__help,status)
                cmd="nvctl__subcmd__kde__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__monitor,export)
                cmd="nvctl__subcmd__monitor__subcmd__export"
                ;;
            nvctl__subcmd__monitor,help)
                cmd="nvctl__subcmd__monitor__subcmd__help"
                ;;
            nvctl__subcmd__monitor,start)
                cmd="nvctl__subcmd__monitor__subcmd__start"
                ;;
            nvctl__subcmd__monitor,status)
                cmd="nvctl__subcmd__monitor__subcmd__status"
                ;;
            nvctl__subcmd__monitor,stop)
                cmd="nvctl__subcmd__monitor__subcmd__stop"
                ;;
            nvctl__subcmd__monitor,tui)
                cmd="nvctl__subcmd__monitor__subcmd__tui"
                ;;
            nvctl__subcmd__monitor__subcmd__help,export)
                cmd="nvctl__subcmd__monitor__subcmd__help__subcmd__export"
                ;;
            nvctl__subcmd__monitor__subcmd__help,help)
                cmd="nvctl__subcmd__monitor__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__monitor__subcmd__help,start)
                cmd="nvctl__subcmd__monitor__subcmd__help__subcmd__start"
                ;;
            nvctl__subcmd__monitor__subcmd__help,status)
                cmd="nvctl__subcmd__monitor__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__monitor__subcmd__help,stop)
                cmd="nvctl__subcmd__monitor__subcmd__help__subcmd__stop"
                ;;
            nvctl__subcmd__monitor__subcmd__help,tui)
                cmd="nvctl__subcmd__monitor__subcmd__help__subcmd__tui"
                ;;
            nvctl__subcmd__monitors,apply-preset)
                cmd="nvctl__subcmd__monitors__subcmd__apply__subcmd__preset"
                ;;
            nvctl__subcmd__monitors,auto)
                cmd="nvctl__subcmd__monitors__subcmd__auto"
                ;;
            nvctl__subcmd__monitors,create-examples)
                cmd="nvctl__subcmd__monitors__subcmd__create__subcmd__examples"
                ;;
            nvctl__subcmd__monitors,gamescope)
                cmd="nvctl__subcmd__monitors__subcmd__gamescope"
                ;;
            nvctl__subcmd__monitors,help)
                cmd="nvctl__subcmd__monitors__subcmd__help"
                ;;
            nvctl__subcmd__monitors,list)
                cmd="nvctl__subcmd__monitors__subcmd__list"
                ;;
            nvctl__subcmd__monitors,load)
                cmd="nvctl__subcmd__monitors__subcmd__load"
                ;;
            nvctl__subcmd__monitors,presets)
                cmd="nvctl__subcmd__monitors__subcmd__presets"
                ;;
            nvctl__subcmd__monitors,preview)
                cmd="nvctl__subcmd__monitors__subcmd__preview"
                ;;
            nvctl__subcmd__monitors,save)
                cmd="nvctl__subcmd__monitors__subcmd__save"
                ;;
            nvctl__subcmd__monitors,set-vrr)
                cmd="nvctl__subcmd__monitors__subcmd__set__subcmd__vrr"
                ;;
            nvctl__subcmd__monitors,status)
                cmd="nvctl__subcmd__monitors__subcmd__status"
                ;;
            nvctl__subcmd__monitors,suggest)
                cmd="nvctl__subcmd__monitors__subcmd__suggest"
                ;;
            nvctl__subcmd__monitors__subcmd__help,apply-preset)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__apply__subcmd__preset"
                ;;
            nvctl__subcmd__monitors__subcmd__help,auto)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__auto"
                ;;
            nvctl__subcmd__monitors__subcmd__help,create-examples)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__create__subcmd__examples"
                ;;
            nvctl__subcmd__monitors__subcmd__help,gamescope)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__gamescope"
                ;;
            nvctl__subcmd__monitors__subcmd__help,help)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__monitors__subcmd__help,list)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__monitors__subcmd__help,load)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__load"
                ;;
            nvctl__subcmd__monitors__subcmd__help,presets)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__presets"
                ;;
            nvctl__subcmd__monitors__subcmd__help,preview)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__preview"
                ;;
            nvctl__subcmd__monitors__subcmd__help,save)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__save"
                ;;
            nvctl__subcmd__monitors__subcmd__help,set-vrr)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__set__subcmd__vrr"
                ;;
            nvctl__subcmd__monitors__subcmd__help,status)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__monitors__subcmd__help,suggest)
                cmd="nvctl__subcmd__monitors__subcmd__help__subcmd__suggest"
                ;;
            nvctl__subcmd__osd,add)
                cmd="nvctl__subcmd__osd__subcmd__add"
                ;;
            nvctl__subcmd__osd,check)
                cmd="nvctl__subcmd__osd__subcmd__check"
                ;;
            nvctl__subcmd__osd,config)
                cmd="nvctl__subcmd__osd__subcmd__config"
                ;;
            nvctl__subcmd__osd,disable)
                cmd="nvctl__subcmd__osd__subcmd__disable"
                ;;
            nvctl__subcmd__osd,enable)
                cmd="nvctl__subcmd__osd__subcmd__enable"
                ;;
            nvctl__subcmd__osd,help)
                cmd="nvctl__subcmd__osd__subcmd__help"
                ;;
            nvctl__subcmd__osd,metrics)
                cmd="nvctl__subcmd__osd__subcmd__metrics"
                ;;
            nvctl__subcmd__osd,remove)
                cmd="nvctl__subcmd__osd__subcmd__remove"
                ;;
            nvctl__subcmd__osd,status)
                cmd="nvctl__subcmd__osd__subcmd__status"
                ;;
            nvctl__subcmd__osd__subcmd__help,add)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__add"
                ;;
            nvctl__subcmd__osd__subcmd__help,check)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__check"
                ;;
            nvctl__subcmd__osd__subcmd__help,config)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__config"
                ;;
            nvctl__subcmd__osd__subcmd__help,disable)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__osd__subcmd__help,enable)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__osd__subcmd__help,help)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__osd__subcmd__help,metrics)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__metrics"
                ;;
            nvctl__subcmd__osd__subcmd__help,remove)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__remove"
                ;;
            nvctl__subcmd__osd__subcmd__help,status)
                cmd="nvctl__subcmd__osd__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__overclock,apply)
                cmd="nvctl__subcmd__overclock__subcmd__apply"
                ;;
            nvctl__subcmd__overclock,auto)
                cmd="nvctl__subcmd__overclock__subcmd__auto"
                ;;
            nvctl__subcmd__overclock,help)
                cmd="nvctl__subcmd__overclock__subcmd__help"
                ;;
            nvctl__subcmd__overclock,info)
                cmd="nvctl__subcmd__overclock__subcmd__info"
                ;;
            nvctl__subcmd__overclock,profile)
                cmd="nvctl__subcmd__overclock__subcmd__profile"
                ;;
            nvctl__subcmd__overclock,reset)
                cmd="nvctl__subcmd__overclock__subcmd__reset"
                ;;
            nvctl__subcmd__overclock,stress-test)
                cmd="nvctl__subcmd__overclock__subcmd__stress__subcmd__test"
                ;;
            nvctl__subcmd__overclock__subcmd__help,apply)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__apply"
                ;;
            nvctl__subcmd__overclock__subcmd__help,auto)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__auto"
                ;;
            nvctl__subcmd__overclock__subcmd__help,help)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__overclock__subcmd__help,info)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__overclock__subcmd__help,profile)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__profile"
                ;;
            nvctl__subcmd__overclock__subcmd__help,reset)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__reset"
                ;;
            nvctl__subcmd__overclock__subcmd__help,stress-test)
                cmd="nvctl__subcmd__overclock__subcmd__help__subcmd__stress__subcmd__test"
                ;;
            nvctl__subcmd__passthrough,bind-vfio)
                cmd="nvctl__subcmd__passthrough__subcmd__bind__subcmd__vfio"
                ;;
            nvctl__subcmd__passthrough,help)
                cmd="nvctl__subcmd__passthrough__subcmd__help"
                ;;
            nvctl__subcmd__passthrough,hugepages)
                cmd="nvctl__subcmd__passthrough__subcmd__hugepages"
                ;;
            nvctl__subcmd__passthrough,iommu)
                cmd="nvctl__subcmd__passthrough__subcmd__iommu"
                ;;
            nvctl__subcmd__passthrough,list)
                cmd="nvctl__subcmd__passthrough__subcmd__list"
                ;;
            nvctl__subcmd__passthrough,persistent)
                cmd="nvctl__subcmd__passthrough__subcmd__persistent"
                ;;
            nvctl__subcmd__passthrough,qemu-command)
                cmd="nvctl__subcmd__passthrough__subcmd__qemu__subcmd__command"
                ;;
            nvctl__subcmd__passthrough,status)
                cmd="nvctl__subcmd__passthrough__subcmd__status"
                ;;
            nvctl__subcmd__passthrough,test-container)
                cmd="nvctl__subcmd__passthrough__subcmd__test__subcmd__container"
                ;;
            nvctl__subcmd__passthrough,unbind-vfio)
                cmd="nvctl__subcmd__passthrough__subcmd__unbind__subcmd__vfio"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,bind-vfio)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__bind__subcmd__vfio"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,help)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,hugepages)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__hugepages"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,iommu)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__iommu"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,list)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,persistent)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__persistent"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,qemu-command)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__qemu__subcmd__command"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,status)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,test-container)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__test__subcmd__container"
                ;;
            nvctl__subcmd__passthrough__subcmd__help,unbind-vfio)
                cmd="nvctl__subcmd__passthrough__subcmd__help__subcmd__unbind__subcmd__vfio"
                ;;
            nvctl__subcmd__power,automate)
                cmd="nvctl__subcmd__power__subcmd__automate"
                ;;
            nvctl__subcmd__power,curve)
                cmd="nvctl__subcmd__power__subcmd__curve"
                ;;
            nvctl__subcmd__power,help)
                cmd="nvctl__subcmd__power__subcmd__help"
                ;;
            nvctl__subcmd__power,limit)
                cmd="nvctl__subcmd__power__subcmd__limit"
                ;;
            nvctl__subcmd__power,monitor)
                cmd="nvctl__subcmd__power__subcmd__monitor"
                ;;
            nvctl__subcmd__power,persistence)
                cmd="nvctl__subcmd__power__subcmd__persistence"
                ;;
            nvctl__subcmd__power,profile)
                cmd="nvctl__subcmd__power__subcmd__profile"
                ;;
            nvctl__subcmd__power,schedule)
                cmd="nvctl__subcmd__power__subcmd__schedule"
                ;;
            nvctl__subcmd__power,status)
                cmd="nvctl__subcmd__power__subcmd__status"
                ;;
            nvctl__subcmd__power__subcmd__curve,add)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__add"
                ;;
            nvctl__subcmd__power__subcmd__curve,disable)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__disable"
                ;;
            nvctl__subcmd__power__subcmd__curve,edit)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__edit"
                ;;
            nvctl__subcmd__power__subcmd__curve,enable)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__enable"
                ;;
            nvctl__subcmd__power__subcmd__curve,help)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__curve,remove)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__remove"
                ;;
            nvctl__subcmd__power__subcmd__curve,reset)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__reset"
                ;;
            nvctl__subcmd__power__subcmd__curve,show)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__show"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,add)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__add"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,disable)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,edit)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__edit"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,enable)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,help)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,remove)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__remove"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,reset)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__reset"
                ;;
            nvctl__subcmd__power__subcmd__curve__subcmd__help,show)
                cmd="nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__show"
                ;;
            nvctl__subcmd__power__subcmd__help,automate)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__automate"
                ;;
            nvctl__subcmd__power__subcmd__help,curve)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve"
                ;;
            nvctl__subcmd__power__subcmd__help,help)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__help,limit)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__limit"
                ;;
            nvctl__subcmd__power__subcmd__help,monitor)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__monitor"
                ;;
            nvctl__subcmd__power__subcmd__help,persistence)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__persistence"
                ;;
            nvctl__subcmd__power__subcmd__help,profile)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__profile"
                ;;
            nvctl__subcmd__power__subcmd__help,schedule)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__schedule"
                ;;
            nvctl__subcmd__power__subcmd__help,status)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,add)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__add"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,disable)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__disable"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,edit)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__edit"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,enable)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__enable"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,remove)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__remove"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,reset)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__reset"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__curve,show)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__show"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__schedule,add)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__add"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__schedule,disable)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__disable"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__schedule,enable)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__enable"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__schedule,list)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__list"
                ;;
            nvctl__subcmd__power__subcmd__help__subcmd__schedule,remove)
                cmd="nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__remove"
                ;;
            nvctl__subcmd__power__subcmd__profile,apply)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__apply"
                ;;
            nvctl__subcmd__power__subcmd__profile,auto-power)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__auto__subcmd__power"
                ;;
            nvctl__subcmd__power__subcmd__profile,create-activity)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__create__subcmd__activity"
                ;;
            nvctl__subcmd__power__subcmd__profile,create-defaults)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__create__subcmd__defaults"
                ;;
            nvctl__subcmd__power__subcmd__profile,help)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__profile,idle)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__idle"
                ;;
            nvctl__subcmd__power__subcmd__profile,monitor)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__monitor"
                ;;
            nvctl__subcmd__power__subcmd__profile,set)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__set"
                ;;
            nvctl__subcmd__power__subcmd__profile,status)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__status"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,apply)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__apply"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,auto-power)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__auto__subcmd__power"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,create-activity)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__create__subcmd__activity"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,create-defaults)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__create__subcmd__defaults"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,help)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,idle)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__idle"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,monitor)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__monitor"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,set)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__set"
                ;;
            nvctl__subcmd__power__subcmd__profile__subcmd__help,status)
                cmd="nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__power__subcmd__schedule,add)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__add"
                ;;
            nvctl__subcmd__power__subcmd__schedule,disable)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__disable"
                ;;
            nvctl__subcmd__power__subcmd__schedule,enable)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__enable"
                ;;
            nvctl__subcmd__power__subcmd__schedule,help)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__schedule,list)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__list"
                ;;
            nvctl__subcmd__power__subcmd__schedule,remove)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__remove"
                ;;
            nvctl__subcmd__power__subcmd__schedule__subcmd__help,add)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__add"
                ;;
            nvctl__subcmd__power__subcmd__schedule__subcmd__help,disable)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__power__subcmd__schedule__subcmd__help,enable)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__power__subcmd__schedule__subcmd__help,help)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__power__subcmd__schedule__subcmd__help,list)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__list"
                ;;
            nvctl__subcmd__power__subcmd__schedule__subcmd__help,remove)
                cmd="nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__remove"
                ;;
            nvctl__subcmd__recording,help)
                cmd="nvctl__subcmd__recording__subcmd__help"
                ;;
            nvctl__subcmd__recording,instant-replay)
                cmd="nvctl__subcmd__recording__subcmd__instant__subcmd__replay"
                ;;
            nvctl__subcmd__recording,presets)
                cmd="nvctl__subcmd__recording__subcmd__presets"
                ;;
            nvctl__subcmd__recording,save)
                cmd="nvctl__subcmd__recording__subcmd__save"
                ;;
            nvctl__subcmd__recording,start)
                cmd="nvctl__subcmd__recording__subcmd__start"
                ;;
            nvctl__subcmd__recording,status)
                cmd="nvctl__subcmd__recording__subcmd__status"
                ;;
            nvctl__subcmd__recording,stop)
                cmd="nvctl__subcmd__recording__subcmd__stop"
                ;;
            nvctl__subcmd__recording__subcmd__help,help)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__recording__subcmd__help,instant-replay)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__instant__subcmd__replay"
                ;;
            nvctl__subcmd__recording__subcmd__help,presets)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__presets"
                ;;
            nvctl__subcmd__recording__subcmd__help,save)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__save"
                ;;
            nvctl__subcmd__recording__subcmd__help,start)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__start"
                ;;
            nvctl__subcmd__recording__subcmd__help,status)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__recording__subcmd__help,stop)
                cmd="nvctl__subcmd__recording__subcmd__help__subcmd__stop"
                ;;
            nvctl__subcmd__shaders,clear)
                cmd="nvctl__subcmd__shaders__subcmd__clear"
                ;;
            nvctl__subcmd__shaders,help)
                cmd="nvctl__subcmd__shaders__subcmd__help"
                ;;
            nvctl__subcmd__shaders,open)
                cmd="nvctl__subcmd__shaders__subcmd__open"
                ;;
            nvctl__subcmd__shaders,optimize)
                cmd="nvctl__subcmd__shaders__subcmd__optimize"
                ;;
            nvctl__subcmd__shaders,precompile)
                cmd="nvctl__subcmd__shaders__subcmd__precompile"
                ;;
            nvctl__subcmd__shaders,stats)
                cmd="nvctl__subcmd__shaders__subcmd__stats"
                ;;
            nvctl__subcmd__shaders__subcmd__help,clear)
                cmd="nvctl__subcmd__shaders__subcmd__help__subcmd__clear"
                ;;
            nvctl__subcmd__shaders__subcmd__help,help)
                cmd="nvctl__subcmd__shaders__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__shaders__subcmd__help,open)
                cmd="nvctl__subcmd__shaders__subcmd__help__subcmd__open"
                ;;
            nvctl__subcmd__shaders__subcmd__help,optimize)
                cmd="nvctl__subcmd__shaders__subcmd__help__subcmd__optimize"
                ;;
            nvctl__subcmd__shaders__subcmd__help,precompile)
                cmd="nvctl__subcmd__shaders__subcmd__help__subcmd__precompile"
                ;;
            nvctl__subcmd__shaders__subcmd__help,stats)
                cmd="nvctl__subcmd__shaders__subcmd__help__subcmd__stats"
                ;;
            nvctl__subcmd__system,compositor)
                cmd="nvctl__subcmd__system__subcmd__compositor"
                ;;
            nvctl__subcmd__system,distro)
                cmd="nvctl__subcmd__system__subcmd__distro"
                ;;
            nvctl__subcmd__system,help)
                cmd="nvctl__subcmd__system__subcmd__help"
                ;;
            nvctl__subcmd__system,info)
                cmd="nvctl__subcmd__system__subcmd__info"
                ;;
            nvctl__subcmd__system,optimize)
                cmd="nvctl__subcmd__system__subcmd__optimize"
                ;;
            nvctl__subcmd__system__subcmd__help,compositor)
                cmd="nvctl__subcmd__system__subcmd__help__subcmd__compositor"
                ;;
            nvctl__subcmd__system__subcmd__help,distro)
                cmd="nvctl__subcmd__system__subcmd__help__subcmd__distro"
                ;;
            nvctl__subcmd__system__subcmd__help,help)
                cmd="nvctl__subcmd__system__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__system__subcmd__help,info)
                cmd="nvctl__subcmd__system__subcmd__help__subcmd__info"
                ;;
            nvctl__subcmd__system__subcmd__help,optimize)
                cmd="nvctl__subcmd__system__subcmd__help__subcmd__optimize"
                ;;
            nvctl__subcmd__upscaling,auto-detect)
                cmd="nvctl__subcmd__upscaling__subcmd__auto__subcmd__detect"
                ;;
            nvctl__subcmd__upscaling,disable)
                cmd="nvctl__subcmd__upscaling__subcmd__disable"
                ;;
            nvctl__subcmd__upscaling,enable)
                cmd="nvctl__subcmd__upscaling__subcmd__enable"
                ;;
            nvctl__subcmd__upscaling,help)
                cmd="nvctl__subcmd__upscaling__subcmd__help"
                ;;
            nvctl__subcmd__upscaling,profiles)
                cmd="nvctl__subcmd__upscaling__subcmd__profiles"
                ;;
            nvctl__subcmd__upscaling,status)
                cmd="nvctl__subcmd__upscaling__subcmd__status"
                ;;
            nvctl__subcmd__upscaling__subcmd__help,auto-detect)
                cmd="nvctl__subcmd__upscaling__subcmd__help__subcmd__auto__subcmd__detect"
                ;;
            nvctl__subcmd__upscaling__subcmd__help,disable)
                cmd="nvctl__subcmd__upscaling__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__upscaling__subcmd__help,enable)
                cmd="nvctl__subcmd__upscaling__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__upscaling__subcmd__help,help)
                cmd="nvctl__subcmd__upscaling__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__upscaling__subcmd__help,profiles)
                cmd="nvctl__subcmd__upscaling__subcmd__help__subcmd__profiles"
                ;;
            nvctl__subcmd__upscaling__subcmd__help,status)
                cmd="nvctl__subcmd__upscaling__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__vrr,configure)
                cmd="nvctl__subcmd__vrr__subcmd__configure"
                ;;
            nvctl__subcmd__vrr,disable)
                cmd="nvctl__subcmd__vrr__subcmd__disable"
                ;;
            nvctl__subcmd__vrr,enable)
                cmd="nvctl__subcmd__vrr__subcmd__enable"
                ;;
            nvctl__subcmd__vrr,help)
                cmd="nvctl__subcmd__vrr__subcmd__help"
                ;;
            nvctl__subcmd__vrr,status)
                cmd="nvctl__subcmd__vrr__subcmd__status"
                ;;
            nvctl__subcmd__vrr__subcmd__help,configure)
                cmd="nvctl__subcmd__vrr__subcmd__help__subcmd__configure"
                ;;
            nvctl__subcmd__vrr__subcmd__help,disable)
                cmd="nvctl__subcmd__vrr__subcmd__help__subcmd__disable"
                ;;
            nvctl__subcmd__vrr__subcmd__help,enable)
                cmd="nvctl__subcmd__vrr__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__vrr__subcmd__help,help)
                cmd="nvctl__subcmd__vrr__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__vrr__subcmd__help,status)
                cmd="nvctl__subcmd__vrr__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__wayland,explicit-sync)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync"
                ;;
            nvctl__subcmd__wayland,export-env)
                cmd="nvctl__subcmd__wayland__subcmd__export__subcmd__env"
                ;;
            nvctl__subcmd__wayland,help)
                cmd="nvctl__subcmd__wayland__subcmd__help"
                ;;
            nvctl__subcmd__wayland,optimize)
                cmd="nvctl__subcmd__wayland__subcmd__optimize"
                ;;
            nvctl__subcmd__wayland,status)
                cmd="nvctl__subcmd__wayland__subcmd__status"
                ;;
            nvctl__subcmd__wayland,switch-driver)
                cmd="nvctl__subcmd__wayland__subcmd__switch__subcmd__driver"
                ;;
            nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync,enable)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__enable"
                ;;
            nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync,help)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help"
                ;;
            nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync,status)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__status"
                ;;
            nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help,enable)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help__subcmd__enable"
                ;;
            nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help,help)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help,status)
                cmd="nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__wayland__subcmd__help,explicit-sync)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync"
                ;;
            nvctl__subcmd__wayland__subcmd__help,export-env)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__export__subcmd__env"
                ;;
            nvctl__subcmd__wayland__subcmd__help,help)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__help"
                ;;
            nvctl__subcmd__wayland__subcmd__help,optimize)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__optimize"
                ;;
            nvctl__subcmd__wayland__subcmd__help,status)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__status"
                ;;
            nvctl__subcmd__wayland__subcmd__help,switch-driver)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__switch__subcmd__driver"
                ;;
            nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync,enable)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync__subcmd__enable"
                ;;
            nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync,status)
                cmd="nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync__subcmd__status"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        nvctl)
            opts="-v -h -V --verbose --format --no-color --help --version gpu display vibrance fan overclock vrr monitor tui nvtop gaming recording container driver power color config upscaling dlss shaders passthrough wayland kde hdr power-profile arch monitors osd interactive system doctor completion version asus companion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch)
            opts="-v -h --verbose --format --no-color --help status install-hooks remove-hooks rebuild-dkms mkinitcpio check-updates aur-suggestions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__aur__subcmd__suggestions)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__check__subcmd__updates)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help)
            opts="status install-hooks remove-hooks rebuild-dkms mkinitcpio check-updates aur-suggestions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__aur__subcmd__suggestions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__check__subcmd__updates)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__install__subcmd__hooks)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__mkinitcpio)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__rebuild__subcmd__dkms)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__remove__subcmd__hooks)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__install__subcmd__hooks)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__mkinitcpio)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__rebuild__subcmd__dkms)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__remove__subcmd__hooks)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__arch__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus)
            opts="-v -h --verbose --format --no-color --help detect power status aura help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura)
            opts="-v -h --verbose --format --no-color --help status mode color temp-reactive restore help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__color)
            opts="-v -h --verbose --format --no-color --help <COLOR>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help)
            opts="status mode color temp-reactive restore help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__color)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__mode)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__restore)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__temp__subcmd__reactive)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__mode)
            opts="-v -h --verbose --format --no-color --help <MODE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__restore)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__aura__subcmd__temp__subcmd__reactive)
            opts="-v -h --enabled --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__detect)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help)
            opts="detect power status aura help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__aura)
            opts="status mode color temp-reactive restore"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__color)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__mode)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__restore)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__temp__subcmd__reactive)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__detect)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__power)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__power)
            opts="-g -w -v -h --gpu --json --watch --interval --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --gpu)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -g)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__asus__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color)
            opts="-v -h --verbose --format --no-color --help vibrance profiles help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help)
            opts="vibrance profiles help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__profiles)
            opts="list create apply schedule"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__schedule)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__vibrance)
            opts="get set apply preview"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles)
            opts="-v -h --verbose --format --no-color --help list create apply schedule help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__apply)
            opts="-n -v -h --name --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__create)
            opts="-n -v -h --name --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__help)
            opts="list create apply schedule help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__schedule)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__profiles__subcmd__schedule)
            opts="-n -t -v -h --name --time --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --time)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance)
            opts="-v -h --verbose --format --no-color --help get set apply preview help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__apply)
            opts="-p -v -h --profile --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__get)
            opts="-d -v -h --display --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__help)
            opts="get set apply preview help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__preview)
            opts="-p -d -v -h --profile --duration --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__color__subcmd__vibrance__subcmd__set)
            opts="-d -v -h --value --display --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --value)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --display)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion)
            opts="-v -h --verbose --format --no-color --help notify-test open-docs help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion__subcmd__help)
            opts="notify-test open-docs help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion__subcmd__help__subcmd__notify__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion__subcmd__help__subcmd__open__subcmd__docs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion__subcmd__notify__subcmd__test)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__companion__subcmd__open__subcmd__docs)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__completion)
            opts="-v -h --verbose --format --no-color --help bash zsh fish"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config)
            opts="-v -h --verbose --format --no-color --help show edit reset backup restore export import capture preview diff apply profiles help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__apply)
            opts="-i -v -h --input --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --input)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__backup)
            opts="-o -v -h --output --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__capture)
            opts="-n -v -h --name --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__diff)
            opts="-v -h --current --target --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --current)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --target)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__edit)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__export)
            opts="-p -o -v -h --profile --output --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help)
            opts="show edit reset backup restore export import capture preview diff apply profiles help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__backup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__capture)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__edit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__profiles)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__restore)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__help__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__import)
            opts="-i -n -v -h --input --name --skip-validation --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --input)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__preview)
            opts="-i -v -h --input --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --input)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__profiles)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__reset)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__restore)
            opts="-i -v -h --input --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --input)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__config__subcmd__show)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container)
            opts="-v -h --verbose --format --no-color --help list status monitor launch phantom-link profiles runtime help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help)
            opts="list status monitor launch phantom-link profiles runtime help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__launch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__monitor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__phantom__subcmd__link)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__profiles)
            opts="list apply create"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__runtime)
            opts="info doctor setup test configure"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__configure)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__launch)
            opts="-m -n -g -i -r -v -h --image --name --gpu --interactive --rm --runtime --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --image)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --gpu)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -g)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --runtime)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__monitor)
            opts="-c -i -v -h --container --interval --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --container)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__phantom__subcmd__link)
            opts="-m -a -v -h --mode --audio-device --rtx-voice --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --mode)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --audio-device)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles)
            opts="-v -h --verbose --format --no-color --help list apply create help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__apply)
            opts="-p -c -v -h --profile --container --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --container)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__create)
            opts="-n -w -v -h --name --workload --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --workload)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -w)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__help)
            opts="list apply create help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__profiles__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime)
            opts="-v -h --verbose --format --no-color --help info doctor setup test configure help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__configure)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__doctor)
            opts="-r -v -h --runtime --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --runtime)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help)
            opts="info doctor setup test configure help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__configure)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__info)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__setup)
            opts="-r -v -h --runtime --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --runtime)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__runtime__subcmd__test)
            opts="-r -v -h --runtime --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --runtime)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__container__subcmd__status)
            opts="-c -v -h --container --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --container)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display)
            opts="-v -h --verbose --format --no-color --help info ls vibrance hdr gamma sharpening color-range color-space dithering help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range)
            opts="-v -h --verbose --format --no-color --help get set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__get)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help)
            opts="get set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__range__subcmd__set)
            opts="-d -v -h --display-id --verbose --format --no-color --help full limited"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space)
            opts="-v -h --verbose --format --no-color --help get set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__get)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help)
            opts="get set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__color__subcmd__space__subcmd__set)
            opts="-d -v -h --display-id --verbose --format --no-color --help rgb ycbcr422 ycbcr444"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering)
            opts="-v -h --verbose --format --no-color --help get enable disable help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__disable)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__enable)
            opts="-d -v -h --display-id --mode --depth --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --mode)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --depth)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__get)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__help)
            opts="get enable disable help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma)
            opts="-v -h --verbose --format --no-color --help get set reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__get)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__help)
            opts="get set reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__reset)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__gamma__subcmd__set)
            opts="-d -v -h --display-id --verbose --format --no-color --help <GAMMA>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr)
            opts="-v -h --verbose --format --no-color --help status enable disable toggle help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help <DISPLAY_ID>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help <DISPLAY_ID>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__help)
            opts="status enable disable toggle help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__toggle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__hdr__subcmd__toggle)
            opts="-v -h --verbose --format --no-color --help <DISPLAY_ID>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help)
            opts="info ls vibrance hdr gamma sharpening color-range color-space dithering help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range)
            opts="get set"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__range__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space)
            opts="get set"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__color__subcmd__space__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__dithering)
            opts="get enable disable"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__gamma)
            opts="get set reset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__hdr)
            opts="status enable disable toggle"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__toggle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__ls)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__sharpening)
            opts="get set reset info"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance)
            opts="get set set-display set-raw list reset info"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set__subcmd__display)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set__subcmd__raw)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__info)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__ls)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening)
            opts="-v -h --verbose --format --no-color --help get set reset info help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__get)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__help)
            opts="get set reset info help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__info)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__reset)
            opts="-d -v -h --display-id --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__sharpening__subcmd__set)
            opts="-d -v -h --display-id --value --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --display-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --value)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance)
            opts="-v -h --verbose --format --no-color --help get set set-display set-raw list reset info help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__get)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help)
            opts="get set set-display set-raw list reset info help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set__subcmd__display)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set__subcmd__raw)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__info)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__reset)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__set)
            opts="-v -h --verbose --format --no-color --help <PERCENTAGE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__display)
            opts="-v -h --verbose --format --no-color --help <DISPLAY> <PERCENTAGE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__raw)
            opts="-v -h --verbose --format --no-color --help <LEVELS>..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss)
            opts="-v -h --verbose --format --no-color --help status enable disable profiles auto metrics doctor games versions launch-opts info help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__auto)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__doctor)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__enable)
            opts="-v -h --quality --frame-generation --reflex --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --quality)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__games)
            opts="-v -h --outdated --json --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help)
            opts="status enable disable profiles auto metrics doctor games versions launch-opts info help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__auto)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__games)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__launch__subcmd__opts)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__metrics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__profiles)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__help__subcmd__versions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__info)
            opts="-v -h --verbose --format --no-color --help <GAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__launch__subcmd__opts)
            opts="-v -h --indicator --version --verbose --format --no-color --help <GAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__metrics)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__profiles)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__dlss__subcmd__versions)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__doctor)
            opts="-v -h --support --format --output --verbose --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver)
            opts="-v -h --verbose --format --no-color --help info check capabilities validate diagnose-release support-bundle install update rollback dkms gsp logs source help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__capabilities)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__check)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__diagnose__subcmd__release)
            opts="-v -h --format --verbose --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms)
            opts="-v -h --verbose --format --no-color --help status doctor setup build logs unregister hook fix cleanup help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__build)
            opts="-k -f -v -h --kernel --force --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --kernel)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -k)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__cleanup)
            opts="-k -v -h --keep --execute --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --keep)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -k)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__doctor)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__fix)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help)
            opts="status doctor setup build logs unregister hook fix cleanup help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__cleanup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__fix)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__hook)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__logs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__unregister)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__hook)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__logs)
            opts="-k -t -v -h --kernel --tail --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --kernel)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -k)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --tail)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__setup)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__dkms__subcmd__unregister)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp)
            opts="-v -h --verbose --format --no-color --help status enable disable diagnostics explain check-update update help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__check__subcmd__update)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__diagnostics)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__explain)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help)
            opts="status enable disable diagnostics explain check-update update help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__check__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__diagnostics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__explain)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__gsp__subcmd__update)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help)
            opts="info check capabilities validate diagnose-release support-bundle install update rollback dkms gsp logs source help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__diagnose__subcmd__release)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms)
            opts="status doctor setup build logs unregister hook fix cleanup"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__cleanup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__fix)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__hook)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__logs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__unregister)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp)
            opts="status enable disable diagnostics explain check-update update"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__check__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__diagnostics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__explain)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__logs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__rollback)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__source)
            opts="status doctor init update sync"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__sync)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__support__subcmd__bundle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__help__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__info)
            opts="-v -h --paste --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__install)
            opts="-v -h --verbose --format --no-color --help <DRIVER_TYPE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__logs)
            opts="-v -h --filter --tail --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --filter)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --tail)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__rollback)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source)
            opts="-v -h --verbose --format --no-color --help status doctor init update sync help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__doctor)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help)
            opts="status doctor init update sync help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__sync)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__init)
            opts="-v -h --verbose --format --no-color --help <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__sync)
            opts="-k -f -v -h --kernel --force --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --kernel)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -k)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__source__subcmd__update)
            opts="-v -h --no-build --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__support__subcmd__bundle)
            opts="-v -h --output --tarball --gzip --redact-paths --redact-ids --log-tail --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-tail)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__update)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__driver__subcmd__validate)
            opts="-v -h --driver --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --driver)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan)
            opts="-v -h --verbose --format --no-color --help info set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan__subcmd__help)
            opts="info set help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan__subcmd__info)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__fan__subcmd__set)
            opts="-v -h --verbose --format --no-color --help <FAN_ID> <PERCENT>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming)
            opts="-v -h --verbose --format --no-color --help enable disable status latency gamescope launch auto help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto)
            opts="-v -h --verbose --format --no-color --help start stop status install-service uninstall-service enable-service disable-service daemon enable disable config help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__config)
            opts="-v -h --poll-interval --apply-delay --restore-on-exit --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --poll-interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --apply-delay)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --restore-on-exit)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__daemon)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__disable__subcmd__service)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__enable__subcmd__service)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help)
            opts="start stop status install-service uninstall-service enable-service disable-service daemon enable disable config help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__daemon)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__install__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__uninstall__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__install__subcmd__service)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__start)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__stop)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__auto__subcmd__uninstall__subcmd__service)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope)
            opts="-v -h --verbose --format --no-color --help launch presets create-preset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__create__subcmd__preset)
            opts="-n -v -h --name --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help)
            opts="launch presets create-preset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__create__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__launch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__launch)
            opts="-c -p -w -v -h --command --preset --width --height --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --command)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --preset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --width)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -w)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --height)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__gamescope__subcmd__presets)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help)
            opts="enable disable status latency gamescope launch auto help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto)
            opts="start stop status install-service uninstall-service enable-service disable-service daemon enable disable config"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__daemon)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__install__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__uninstall__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope)
            opts="launch presets create-preset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__create__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__launch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__latency)
            opts="optimize status test"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch)
            opts="run list show create delete hook-add hook-list hook-remove set-gamescope-preset examples"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__delete)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__examples)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__set__subcmd__gamescope__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency)
            opts="-v -h --verbose --format --no-color --help optimize status test help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__help)
            opts="optimize status test help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__optimize)
            opts="-p -v -h --preset --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --preset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__latency__subcmd__test)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch)
            opts="-v -h --verbose --format --no-color --help run list show create delete hook-add hook-list hook-remove set-gamescope-preset examples help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__create)
            opts="-v -h --verbose --format --no-color --help <PROFILE> <EXECUTABLE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__delete)
            opts="-v -h --verbose --format --no-color --help <PROFILE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__examples)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help)
            opts="run list show create delete hook-add hook-list hook-remove set-gamescope-preset examples help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__delete)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__examples)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__set__subcmd__gamescope__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__add)
            opts="-v -h --verbose --format --no-color --help <PROFILE> <PHASE> <COMMAND> [ARGS]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__list)
            opts="-v -h --verbose --format --no-color --help <PROFILE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__remove)
            opts="-v -h --verbose --format --no-color --help <PROFILE> <PHASE> <INDEX>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__run)
            opts="-v -h --verbose --format --no-color --help <PROFILE> [ARGS]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__set__subcmd__gamescope__subcmd__preset)
            opts="-v -h --verbose --format --no-color --help <PROFILE> <PRESET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__launch__subcmd__show)
            opts="-v -h --verbose --format --no-color --help <PROFILE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gaming__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu)
            opts="-v -h --verbose --format --no-color --help info stat capabilities list select benchmark watch export stress help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__benchmark)
            opts="-d -t -v -h --duration --test-type --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --test-type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__capabilities)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__export)
            opts="-f -o -d -v -h --format --output --duration --verbose --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help)
            opts="info stat capabilities list select benchmark watch export stress help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__benchmark)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__select)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__stat)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__stress)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__help__subcmd__watch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__info)
            opts="-f -v -h --format --verbose --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__list)
            opts="-f -v -h --format --verbose --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__select)
            opts="-v -h --verbose --format --no-color --help <INDEX>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__stat)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__stress)
            opts="-d -i -l -v -h --duration --intensity --log --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --intensity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__gpu__subcmd__watch)
            opts="-i -c -v -h --interval --count --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --count)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr)
            opts="-v -h --verbose --format --no-color --help status enable disable config set-brightness tools capabilities help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__capabilities)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__config)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help)
            opts="status enable disable config set-brightness tools capabilities help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__set__subcmd__brightness)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__help__subcmd__tools)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__set__subcmd__brightness)
            opts="-v -h --verbose --format --no-color --help <NITS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__hdr__subcmd__tools)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help)
            opts="gpu display vibrance fan overclock vrr monitor tui nvtop gaming recording container driver power color config upscaling dlss shaders passthrough wayland kde hdr power-profile arch monitors osd interactive system doctor completion version asus companion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch)
            opts="status install-hooks remove-hooks rebuild-dkms mkinitcpio check-updates aur-suggestions"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__aur__subcmd__suggestions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__check__subcmd__updates)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__install__subcmd__hooks)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__mkinitcpio)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__rebuild__subcmd__dkms)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__remove__subcmd__hooks)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__arch__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus)
            opts="detect power status aura"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__aura)
            opts="status mode color temp-reactive restore"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__color)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__mode)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__restore)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__temp__subcmd__reactive)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__detect)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__power)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__asus__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color)
            opts="vibrance profiles"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__profiles)
            opts="list create apply schedule"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__schedule)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__vibrance)
            opts="get set apply preview"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__companion)
            opts="notify-test open-docs"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__companion__subcmd__notify__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__companion__subcmd__open__subcmd__docs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__completion)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config)
            opts="show edit reset backup restore export import capture preview diff apply profiles"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__backup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__capture)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__edit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__profiles)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__restore)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__config__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container)
            opts="list status monitor launch phantom-link profiles runtime"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__launch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__monitor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__phantom__subcmd__link)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__profiles)
            opts="list apply create"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__runtime)
            opts="info doctor setup test configure"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__configure)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__container__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display)
            opts="info ls vibrance hdr gamma sharpening color-range color-space dithering"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range)
            opts="get set"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__range__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space)
            opts="get set"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__color__subcmd__space__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__dithering)
            opts="get enable disable"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__gamma)
            opts="get set reset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__hdr)
            opts="status enable disable toggle"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__toggle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__ls)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__sharpening)
            opts="get set reset info"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance)
            opts="get set set-display set-raw list reset info"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__get)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__display)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set__subcmd__raw)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss)
            opts="status enable disable profiles auto metrics doctor games versions launch-opts info"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__auto)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__games)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__launch__subcmd__opts)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__metrics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__profiles)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__dlss__subcmd__versions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver)
            opts="info check capabilities validate diagnose-release support-bundle install update rollback dkms gsp logs source"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__diagnose__subcmd__release)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms)
            opts="status doctor setup build logs unregister hook fix cleanup"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__cleanup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__fix)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__hook)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__logs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__setup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__unregister)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp)
            opts="status enable disable diagnostics explain check-update update"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__check__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__diagnostics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__explain)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__logs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__rollback)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__source)
            opts="status doctor init update sync"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__doctor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__sync)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__support__subcmd__bundle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__driver__subcmd__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__fan)
            opts="info set"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__fan__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__fan__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming)
            opts="enable disable status latency gamescope launch auto"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto)
            opts="start stop status install-service uninstall-service enable-service disable-service daemon enable disable config"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__daemon)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__install__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__uninstall__subcmd__service)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope)
            opts="launch presets create-preset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__create__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__launch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__latency)
            opts="optimize status test"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch)
            opts="run list show create delete hook-add hook-list hook-remove set-gamescope-preset examples"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__delete)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__examples)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__set__subcmd__gamescope__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gaming__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu)
            opts="info stat capabilities list select benchmark watch export stress"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__benchmark)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__select)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__stat)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__stress)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__gpu__subcmd__watch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr)
            opts="status enable disable config set-brightness tools capabilities"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__set__subcmd__brightness)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__hdr__subcmd__tools)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__interactive)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde)
            opts="status gaming productivity power-save setup-env set-vrr restart"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__gaming)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__power__subcmd__save)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__productivity)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__restart)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__set__subcmd__vrr)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__setup__subcmd__env)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__kde__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitor)
            opts="start stop status tui export"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitor__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitor__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitor__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitor__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitor__subcmd__tui)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors)
            opts="status presets suggest preview apply-preset save load list set-vrr gamescope auto create-examples"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__apply__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__auto)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__create__subcmd__examples)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__gamescope)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__load)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__save)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__set__subcmd__vrr)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__monitors__subcmd__suggest)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__nvtop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd)
            opts="enable disable status config add remove metrics check"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__metrics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__osd__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock)
            opts="info apply profile stress-test auto reset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock__subcmd__auto)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock__subcmd__profile)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__overclock__subcmd__stress__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough)
            opts="status list iommu bind-vfio unbind-vfio persistent test-container qemu-command hugepages"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__bind__subcmd__vfio)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__hugepages)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__iommu)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__persistent)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__qemu__subcmd__command)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__test__subcmd__container)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__passthrough__subcmd__unbind__subcmd__vfio)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power)
            opts="status limit profile persistence monitor automate curve schedule"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile)
            opts="status set create-activity apply monitor auto-power idle create-defaults"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__auto__subcmd__power)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__create__subcmd__activity)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__create__subcmd__defaults)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__idle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__monitor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__automate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve)
            opts="show edit add remove enable disable reset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__edit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__limit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__monitor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__persistence)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__profile)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__schedule)
            opts="list add remove enable disable"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__power__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording)
            opts="start stop status instant-replay save presets"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording__subcmd__instant__subcmd__replay)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording__subcmd__save)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__recording__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__shaders)
            opts="stats clear optimize precompile open"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__shaders__subcmd__clear)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__shaders__subcmd__open)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__shaders__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__shaders__subcmd__precompile)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__shaders__subcmd__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__system)
            opts="info compositor distro optimize"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__system__subcmd__compositor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__system__subcmd__distro)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__system__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__system__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__tui)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__upscaling)
            opts="status enable disable profiles auto-detect"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__upscaling__subcmd__auto__subcmd__detect)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__upscaling__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__upscaling__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__upscaling__subcmd__profiles)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__upscaling__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__version)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__vibrance)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__vrr)
            opts="status enable disable configure"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__vrr__subcmd__configure)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__vrr__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__vrr__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__vrr__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland)
            opts="status optimize export-env switch-driver explicit-sync"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync)
            opts="status enable"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__export__subcmd__env)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__help__subcmd__wayland__subcmd__switch__subcmd__driver)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__interactive)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde)
            opts="-v -h --verbose --format --no-color --help status gaming productivity power-save setup-env set-vrr restart help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__gaming)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help)
            opts="status gaming productivity power-save setup-env set-vrr restart help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__gaming)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__power__subcmd__save)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__productivity)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__restart)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__set__subcmd__vrr)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__setup__subcmd__env)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__power__subcmd__save)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__productivity)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__restart)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__set__subcmd__vrr)
            opts="-v -h --enabled --verbose --format --no-color --help <DISPLAY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__setup__subcmd__env)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__kde__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor)
            opts="-v -h --verbose --format --no-color --help start stop status tui export help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__export)
            opts="-o -d -v -h --output --duration --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help)
            opts="start stop status tui export help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help__subcmd__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__help__subcmd__tui)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__start)
            opts="-i -c -v -h --interval --count --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --count)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__stop)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitor__subcmd__tui)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors)
            opts="-v -h --verbose --format --no-color --help status presets suggest preview apply-preset save load list set-vrr gamescope auto create-examples help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__apply__subcmd__preset)
            opts="-v -h --verbose --format --no-color --help <PRESET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__auto)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__create__subcmd__examples)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__gamescope)
            opts="-w -H -r -v -h --width --height --refresh --verbose --format --no-color --help <CONNECTOR> <COMMAND>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --width)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -w)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --height)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --refresh)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help)
            opts="status presets suggest preview apply-preset save load list set-vrr gamescope auto create-examples help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__apply__subcmd__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__auto)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__create__subcmd__examples)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__gamescope)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__load)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__preview)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__save)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__set__subcmd__vrr)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__help__subcmd__suggest)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__load)
            opts="-v -h --verbose --format --no-color --help <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__presets)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__preview)
            opts="-v -h --verbose --format --no-color --help <PRESET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__save)
            opts="-v -h --verbose --format --no-color --help <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__set__subcmd__vrr)
            opts="-v -h --enabled --verbose --format --no-color --help <CONNECTOR>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__monitors__subcmd__suggest)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__nvtop)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd)
            opts="-v -h --verbose --format --no-color --help enable disable status config add remove metrics check help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__add)
            opts="-v -h --verbose --format --no-color --help <METRIC>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__check)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__config)
            opts="-v -h --position --font-size --opacity --interval --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --position)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --font-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --opacity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help)
            opts="enable disable status config add remove metrics check help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__config)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__metrics)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__metrics)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__remove)
            opts="-v -h --verbose --format --no-color --help <METRIC>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__osd__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock)
            opts="-v -h --verbose --format --no-color --help info apply profile stress-test auto reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__apply)
            opts="-v -h --gpu-offset --memory-offset --power-limit --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --gpu-offset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --memory-offset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --power-limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__auto)
            opts="-v -h --target --safety --max-temp --max-power --stability-duration --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --target)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --safety)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-temp)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-power)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stability-duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help)
            opts="info apply profile stress-test auto reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__auto)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__profile)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__help__subcmd__stress__subcmd__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__info)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__profile)
            opts="-v -h --verbose --format --no-color --help <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__reset)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__overclock__subcmd__stress__subcmd__test)
            opts="-v -h --verbose --format --no-color --help [DURATION]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough)
            opts="-v -h --verbose --format --no-color --help status list iommu bind-vfio unbind-vfio persistent test-container qemu-command hugepages help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__bind__subcmd__vfio)
            opts="-v -h --verbose --format --no-color --help <PCI_ADDRESS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help)
            opts="status list iommu bind-vfio unbind-vfio persistent test-container qemu-command hugepages help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__bind__subcmd__vfio)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__hugepages)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__iommu)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__persistent)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__qemu__subcmd__command)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__test__subcmd__container)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__help__subcmd__unbind__subcmd__vfio)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__hugepages)
            opts="-v -h --verbose --format --no-color --help [SIZE_MB]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__iommu)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__persistent)
            opts="-v -h --verbose --format --no-color --help <PCI_ADDRESS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__qemu__subcmd__command)
            opts="-v -h --verbose --format --no-color --help <PCI_ADDRESS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__test__subcmd__container)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__passthrough__subcmd__unbind__subcmd__vfio)
            opts="-v -h --verbose --format --no-color --help <PCI_ADDRESS>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power)
            opts="-v -h --verbose --format --no-color --help status limit profile persistence monitor automate curve schedule help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile)
            opts="-v -h --verbose --format --no-color --help status set create-activity apply monitor auto-power idle create-defaults help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__apply)
            opts="-v -h --verbose --format --no-color --help <ACTIVITY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__auto__subcmd__power)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__create__subcmd__activity)
            opts="-v -h --system-profile --gpu-offset --mem-offset --verbose --format --no-color --help <ACTIVITY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --system-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --gpu-offset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --mem-offset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__create__subcmd__defaults)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help)
            opts="status set create-activity apply monitor auto-power idle create-defaults help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__auto__subcmd__power)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__create__subcmd__activity)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__create__subcmd__defaults)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__idle)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__monitor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__set)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__idle)
            opts="-v -h --timeout --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__monitor)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__set)
            opts="-v -h --verbose --format --no-color --help <PROFILE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__automate)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve)
            opts="-v -h --verbose --format --no-color --help show edit add remove enable disable reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__add)
            opts="-v -h --verbose --format --no-color --help <TEMP> <POWER>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__edit)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help)
            opts="show edit add remove enable disable reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__edit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__remove)
            opts="-v -h --verbose --format --no-color --help <INDEX>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__reset)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__curve__subcmd__show)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help)
            opts="status limit profile persistence monitor automate curve schedule help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__automate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve)
            opts="show edit add remove enable disable reset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__edit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__limit)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__monitor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__persistence)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__profile)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__schedule)
            opts="list add remove enable disable"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__limit)
            opts="-p -v -h --percentage --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --percentage)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__monitor)
            opts="-d -v -h --duration --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__persistence)
            opts="-v -h --enabled --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__profile)
            opts="-p -v -h --profile --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule)
            opts="-v -h --verbose --format --no-color --help list add remove enable disable help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__add)
            opts="-v -h --hour --days --power --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --hour)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --days)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --power)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help)
            opts="list add remove enable disable help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__list)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__schedule__subcmd__remove)
            opts="-v -h --verbose --format --no-color --help <INDEX>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__power__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording)
            opts="-v -h --verbose --format --no-color --help start stop status instant-replay save presets help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help)
            opts="start stop status instant-replay save presets help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__instant__subcmd__replay)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__save)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__help__subcmd__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__instant__subcmd__replay)
            opts="-d -v -h --duration --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --duration)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__presets)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__save)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__start)
            opts="-o -p -q -v -h --output --preset --quality --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --preset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --quality)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -q)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__recording__subcmd__stop)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders)
            opts="-v -h --verbose --format --no-color --help stats clear optimize precompile open help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__clear)
            opts="-v -h --cache-type --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cache-type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help)
            opts="stats clear optimize precompile open help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help__subcmd__clear)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help__subcmd__open)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help__subcmd__precompile)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__help__subcmd__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__open)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__optimize)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__precompile)
            opts="-v -h --verbose --format --no-color --help <GAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__shaders__subcmd__stats)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system)
            opts="-v -h --verbose --format --no-color --help info compositor distro optimize help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__compositor)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__distro)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__help)
            opts="info compositor distro optimize help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__help__subcmd__compositor)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__help__subcmd__distro)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__help__subcmd__info)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__help__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__info)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__system__subcmd__optimize)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__tui)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling)
            opts="-v -h --verbose --format --no-color --help status enable disable profiles auto-detect help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__auto__subcmd__detect)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help <GAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__enable)
            opts="-v -h --tech --quality --verbose --format --no-color --help <GAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --tech)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --quality)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help)
            opts="status enable disable profiles auto-detect help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help__subcmd__auto__subcmd__detect)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help__subcmd__profiles)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__profiles)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__upscaling__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__version)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vibrance)
            opts="-v -h --verbose --format --no-color --help <PERCENTAGE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr)
            opts="-v -h --verbose --format --no-color --help status enable disable configure help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__configure)
            opts="-v -h --min-refresh --max-refresh --verbose --format --no-color --help <DISPLAY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --min-refresh)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-refresh)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__disable)
            opts="-v -h --verbose --format --no-color --help <DISPLAY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help <DISPLAY>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__help)
            opts="status enable disable configure help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__help__subcmd__configure)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__help__subcmd__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__vrr__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland)
            opts="-v -h --verbose --format --no-color --help status optimize export-env switch-driver explicit-sync help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync)
            opts="-v -h --verbose --format --no-color --help status enable help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__enable)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help)
            opts="status enable help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__explicit__subcmd__sync__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__export__subcmd__env)
            opts="-v -h --config --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help)
            opts="status optimize export-env switch-driver explicit-sync help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync)
            opts="status enable"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync__subcmd__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__explicit__subcmd__sync__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__export__subcmd__env)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__optimize)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__help__subcmd__switch__subcmd__driver)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__optimize)
            opts="-v -h --backup --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__status)
            opts="-v -h --verbose --format --no-color --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        nvctl__subcmd__wayland__subcmd__switch__subcmd__driver)
            opts="-v -h --verbose --format --no-color --help <DRIVER>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "human json yaml table" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _nvctl -o nosort -o bashdefault -o default nvctl
else
    complete -F _nvctl -o bashdefault -o default nvctl
fi
