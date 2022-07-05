#!/usr/bin/env bash 


case "$(uname -s)" in
   Darwin)
     OS='Mac'
     ;;
   Linux)
     OS='Linux'
     ;;
   CYGWIN*|MINGW32*|MSYS*|MINGW*)
     OS='Windows'
     ;;
   *)
     OS='Other' 
     ;;
esac


DIR=${PWD##*/}
MATCH="GitHub"

if [ "$OS" = "Windows" ]; then
    if [ "$DIR" = "$MATCH" ]; then
        start "https://www.github.com" 
    else
        start $(git config remote.origin.url)
    fi
elif [ "$OS" = "Windows" ]; then
    if [ "$DIR" = "$MATCH" ]; then
        xdg-open "https://www.github.com" 
    else
        xdg-open $(git config remote.origin.url)
    fi
fi
