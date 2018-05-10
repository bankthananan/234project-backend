<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_ProjectBackend      chars</name>
   <tag></tag>
   <elementGuidId>22139b09-7310-45ae-b327-db43f49b41fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
  
  ProjectBackend
  

  
  
@charset &quot;UTF-8&quot;;
/* You can add global styles to this file, and also import other style files */
a {
    cursor: pointer;
}
.help-block {
    font-size: 12px;
}
/* Validation Error and some media query */
.validation-error
{
    border: 1px solid red;
}
.validation-error:focus
{
    border: 1px solid red !important;
}
.error-message
{
    color:red;
   
}
.has-error .form-control,
.has-success .form-control:focus,
.has-error .form-control:focus {
  -webkit-box-shadow: none;
  box-shadow: none; }
.has-error .form-control-feedback, .has-error .control-label {
    color: #FF3636; }
/* From ionic */
/*!
  Ionicons, v2.0.0
  Created by Ben Sperry for the Ionic Framework, http://ionicons.com/
  https://twitter.com/benjsperry  https://twitter.com/ionicframework
  MIT License: https://github.com/driftyco/ionicons

  Android-style icons originally built by Google’s
  Material Design Icons: https://github.com/google/material-design-icons
  used under CC BY http://creativecommons.org/licenses/by/4.0/
  Modified icons to fit ionicon’s grid from original.
*/
@font-face { font-family: &quot;Ionicons&quot;; src: url(&quot;/assets/fonts/ionicons.eot?v=2.0.0&quot;);
  src: url(&quot;/assets/fonts/ionicons.eot?v=2.0.0#iefix&quot;) format(&quot;embedded-opentype&quot;),
  url(&quot;/assets/fonts/ionicons.ttf?v=2.0.0&quot;) format(&quot;truetype&quot;),
  url(&quot;/assets/fonts/ionicons.woff?v=2.0.0&quot;) format(&quot;woff&quot;),
  url(&quot;/assets/fonts/ionicons.svg?v=2.0.0#Ionicons&quot;) format(&quot;svg&quot;);
  font-weight: normal; font-style: normal; }
.ion, .ionicons, .ion-alert:before, .ion-alert-circled:before, .ion-android-add:before, .ion-android-add-circle:before, .ion-android-alarm-clock:before, .ion-android-alert:before, .ion-android-apps:before, .ion-android-archive:before, .ion-android-arrow-back:before, .ion-android-arrow-down:before, .ion-android-arrow-dropdown:before, .ion-android-arrow-dropdown-circle:before, .ion-android-arrow-dropleft:before, .ion-android-arrow-dropleft-circle:before, .ion-android-arrow-dropright:before, .ion-android-arrow-dropright-circle:before, .ion-android-arrow-dropup:before, .ion-android-arrow-dropup-circle:before, .ion-android-arrow-forward:before, .ion-android-arrow-up:before, .ion-android-attach:before, .ion-android-bar:before, .ion-android-bicycle:before, .ion-android-boat:before, .ion-android-bookmark:before, .ion-android-bulb:before, .ion-android-bus:before, .ion-android-calendar:before, .ion-android-call:before, .ion-android-camera:before, .ion-android-cancel:before, .ion-android-car:before, .ion-android-cart:before, .ion-android-chat:before, .ion-android-checkbox:before, .ion-android-checkbox-blank:before, .ion-android-checkbox-outline:before, .ion-android-checkbox-outline-blank:before, .ion-android-checkmark-circle:before, .ion-android-clipboard:before, .ion-android-close:before, .ion-android-cloud:before, .ion-android-cloud-circle:before, .ion-android-cloud-done:before, .ion-android-cloud-outline:before, .ion-android-color-palette:before, .ion-android-compass:before, .ion-android-contact:before, .ion-android-contacts:before, .ion-android-contract:before, .ion-android-create:before, .ion-android-delete:before, .ion-android-desktop:before, .ion-android-document:before, .ion-android-done:before, .ion-android-done-all:before, .ion-android-download:before, .ion-android-drafts:before, .ion-android-exit:before, .ion-android-expand:before, .ion-android-favorite:before, .ion-android-favorite-outline:before, .ion-android-film:before, .ion-android-folder:before, .ion-android-folder-open:before, .ion-android-funnel:before, .ion-android-globe:before, .ion-android-hand:before, .ion-android-hangout:before, .ion-android-happy:before, .ion-android-home:before, .ion-android-image:before, .ion-android-laptop:before, .ion-android-list:before, .ion-android-locate:before, .ion-android-lock:before, .ion-android-mail:before, .ion-android-map:before, .ion-android-menu:before, .ion-android-microphone:before, .ion-android-microphone-off:before, .ion-android-more-horizontal:before, .ion-android-more-vertical:before, .ion-android-navigate:before, .ion-android-notifications:before, .ion-android-notifications-none:before, .ion-android-notifications-off:before, .ion-android-open:before, .ion-android-options:before, .ion-android-people:before, .ion-android-person:before, .ion-android-person-add:before, .ion-android-phone-landscape:before, .ion-android-phone-portrait:before, .ion-android-pin:before, .ion-android-plane:before, .ion-android-playstore:before, .ion-android-print:before, .ion-android-radio-button-off:before, .ion-android-radio-button-on:before, .ion-android-refresh:before, .ion-android-remove:before, .ion-android-remove-circle:before, .ion-android-restaurant:before, .ion-android-sad:before, .ion-android-search:before, .ion-android-send:before, .ion-android-settings:before, .ion-android-share:before, .ion-android-share-alt:before, .ion-android-star:before, .ion-android-star-half:before, .ion-android-star-outline:before, .ion-android-stopwatch:before, .ion-android-subway:before, .ion-android-sunny:before, .ion-android-sync:before, .ion-android-textsms:before, .ion-android-time:before, .ion-android-train:before, .ion-android-unlock:before, .ion-android-upload:before, .ion-android-volume-down:before, .ion-android-volume-mute:before, .ion-android-volume-off:before, .ion-android-volume-up:before, .ion-android-walk:before, .ion-android-warning:before, .ion-android-watch:before, .ion-android-wifi:before, .ion-aperture:before, .ion-archive:before, .ion-arrow-down-a:before, .ion-arrow-down-b:before, .ion-arrow-down-c:before, .ion-arrow-expand:before, .ion-arrow-graph-down-left:before, .ion-arrow-graph-down-right:before, .ion-arrow-graph-up-left:before, .ion-arrow-graph-up-right:before, .ion-arrow-left-a:before, .ion-arrow-left-b:before, .ion-arrow-left-c:before, .ion-arrow-move:before, .ion-arrow-resize:before, .ion-arrow-return-left:before, .ion-arrow-return-right:before, .ion-arrow-right-a:before, .ion-arrow-right-b:before, .ion-arrow-right-c:before, .ion-arrow-shrink:before, .ion-arrow-swap:before, .ion-arrow-up-a:before, .ion-arrow-up-b:before, .ion-arrow-up-c:before, .ion-asterisk:before, .ion-at:before, .ion-backspace:before, .ion-backspace-outline:before, .ion-bag:before, .ion-battery-charging:before, .ion-battery-empty:before, .ion-battery-full:before, .ion-battery-half:before, .ion-battery-low:before, .ion-beaker:before, .ion-beer:before, .ion-bluetooth:before, .ion-bonfire:before, .ion-bookmark:before, .ion-bowtie:before, .ion-briefcase:before, .ion-bug:before, .ion-calculator:before, .ion-calendar:before, .ion-camera:before, .ion-card:before, .ion-cash:before, .ion-chatbox:before, .ion-chatbox-working:before, .ion-chatboxes:before, .ion-chatbubble:before, .ion-chatbubble-working:before, .ion-chatbubbles:before, .ion-checkmark:before, .ion-checkmark-circled:before, .ion-checkmark-round:before, .ion-chevron-down:before, .ion-chevron-left:before, .ion-chevron-right:before, .ion-chevron-up:before, .ion-clipboard:before, .ion-clock:before, .ion-close:before, .ion-close-circled:before, .ion-close-round:before, .ion-closed-captioning:before, .ion-cloud:before, .ion-code:before, .ion-code-download:before, .ion-code-working:before, .ion-coffee:before, .ion-compass:before, .ion-compose:before, .ion-connection-bars:before, .ion-contrast:before, .ion-crop:before, .ion-cube:before, .ion-disc:before, .ion-document:before, .ion-document-text:before, .ion-drag:before, .ion-earth:before, .ion-easel:before, .ion-edit:before, .ion-egg:before, .ion-eject:before, .ion-email:before, .ion-email-unread:before, .ion-erlenmeyer-flask:before, .ion-erlenmeyer-flask-bubbles:before, .ion-eye:before, .ion-eye-disabled:before, .ion-female:before, .ion-filing:before, .ion-film-marker:before, .ion-fireball:before, .ion-flag:before, .ion-flame:before, .ion-flash:before, .ion-flash-off:before, .ion-folder:before, .ion-fork:before, .ion-fork-repo:before, .ion-forward:before, .ion-funnel:before, .ion-gear-a:before, .ion-gear-b:before, .ion-grid:before, .ion-hammer:before, .ion-happy:before, .ion-happy-outline:before, .ion-headphone:before, .ion-heart:before, .ion-heart-broken:before, .ion-help:before, .ion-help-buoy:before, .ion-help-circled:before, .ion-home:before, .ion-icecream:before, .ion-image:before, .ion-images:before, .ion-information:before, .ion-information-circled:before, .ion-ionic:before, .ion-ios-alarm:before, .ion-ios-alarm-outline:before, .ion-ios-albums:before, .ion-ios-albums-outline:before, .ion-ios-americanfootball:before, .ion-ios-americanfootball-outline:before, .ion-ios-analytics:before, .ion-ios-analytics-outline:before, .ion-ios-arrow-back:before, .ion-ios-arrow-down:before, .ion-ios-arrow-forward:before, .ion-ios-arrow-left:before, .ion-ios-arrow-right:before, .ion-ios-arrow-thin-down:before, .ion-ios-arrow-thin-left:before, .ion-ios-arrow-thin-right:before, .ion-ios-arrow-thin-up:before, .ion-ios-arrow-up:before, .ion-ios-at:before, .ion-ios-at-outline:before, .ion-ios-barcode:before, .ion-ios-barcode-outline:before, .ion-ios-baseball:before, .ion-ios-baseball-outline:before, .ion-ios-basketball:before, .ion-ios-basketball-outline:before, .ion-ios-bell:before, .ion-ios-bell-outline:before, .ion-ios-body:before, .ion-ios-body-outline:before, .ion-ios-bolt:before, .ion-ios-bolt-outline:before, .ion-ios-book:before, .ion-ios-book-outline:before, .ion-ios-bookmarks:before, .ion-ios-bookmarks-outline:before, .ion-ios-box:before, .ion-ios-box-outline:before, .ion-ios-briefcase:before, .ion-ios-briefcase-outline:before, .ion-ios-browsers:before, .ion-ios-browsers-outline:before, .ion-ios-calculator:before, .ion-ios-calculator-outline:before, .ion-ios-calendar:before, .ion-ios-calendar-outline:before, .ion-ios-camera:before, .ion-ios-camera-outline:before, .ion-ios-cart:before, .ion-ios-cart-outline:before, .ion-ios-chatboxes:before, .ion-ios-chatboxes-outline:before, .ion-ios-chatbubble:before, .ion-ios-chatbubble-outline:before, .ion-ios-checkmark:before, .ion-ios-checkmark-empty:before, .ion-ios-checkmark-outline:before, .ion-ios-circle-filled:before, .ion-ios-circle-outline:before, .ion-ios-clock:before, .ion-ios-clock-outline:before, .ion-ios-close:before, .ion-ios-close-empty:before, .ion-ios-close-outline:before, .ion-ios-cloud:before, .ion-ios-cloud-download:before, .ion-ios-cloud-download-outline:before, .ion-ios-cloud-outline:before, .ion-ios-cloud-upload:before, .ion-ios-cloud-upload-outline:before, .ion-ios-cloudy:before, .ion-ios-cloudy-night:before, .ion-ios-cloudy-night-outline:before, .ion-ios-cloudy-outline:before, .ion-ios-cog:before, .ion-ios-cog-outline:before, .ion-ios-color-filter:before, .ion-ios-color-filter-outline:before, .ion-ios-color-wand:before, .ion-ios-color-wand-outline:before, .ion-ios-compose:before, .ion-ios-compose-outline:before, .ion-ios-contact:before, .ion-ios-contact-outline:before, .ion-ios-copy:before, .ion-ios-copy-outline:before, .ion-ios-crop:before, .ion-ios-crop-strong:before, .ion-ios-download:before, .ion-ios-download-outline:before, .ion-ios-drag:before, .ion-ios-email:before, .ion-ios-email-outline:before, .ion-ios-eye:before, .ion-ios-eye-outline:before, .ion-ios-fastforward:before, .ion-ios-fastforward-outline:before, .ion-ios-filing:before, .ion-ios-filing-outline:before, .ion-ios-film:before, .ion-ios-film-outline:before, .ion-ios-flag:before, .ion-ios-flag-outline:before, .ion-ios-flame:before, .ion-ios-flame-outline:before, .ion-ios-flask:before, .ion-ios-flask-outline:before, .ion-ios-flower:before, .ion-ios-flower-outline:before, .ion-ios-folder:before, .ion-ios-folder-outline:before, .ion-ios-football:before, .ion-ios-football-outline:before, .ion-ios-game-controller-a:before, .ion-ios-game-controller-a-outline:before, .ion-ios-game-controller-b:before, .ion-ios-game-controller-b-outline:before, .ion-ios-gear:before, .ion-ios-gear-outline:before, .ion-ios-glasses:before, .ion-ios-glasses-outline:before, .ion-ios-grid-view:before, .ion-ios-grid-view-outline:before, .ion-ios-heart:before, .ion-ios-heart-outline:before, .ion-ios-help:before, .ion-ios-help-empty:before, .ion-ios-help-outline:before, .ion-ios-home:before, .ion-ios-home-outline:before, .ion-ios-infinite:before, .ion-ios-infinite-outline:before, .ion-ios-information:before, .ion-ios-information-empty:before, .ion-ios-information-outline:before, .ion-ios-ionic-outline:before, .ion-ios-keypad:before, .ion-ios-keypad-outline:before, .ion-ios-lightbulb:before, .ion-ios-lightbulb-outline:before, .ion-ios-list:before, .ion-ios-list-outline:before, .ion-ios-location:before, .ion-ios-location-outline:before, .ion-ios-locked:before, .ion-ios-locked-outline:before, .ion-ios-loop:before, .ion-ios-loop-strong:before, .ion-ios-medical:before, .ion-ios-medical-outline:before, .ion-ios-medkit:before, .ion-ios-medkit-outline:before, .ion-ios-mic:before, .ion-ios-mic-off:before, .ion-ios-mic-outline:before, .ion-ios-minus:before, .ion-ios-minus-empty:before, .ion-ios-minus-outline:before, .ion-ios-monitor:before, .ion-ios-monitor-outline:before, .ion-ios-moon:before, .ion-ios-moon-outline:before, .ion-ios-more:before, .ion-ios-more-outline:before, .ion-ios-musical-note:before, .ion-ios-musical-notes:before, .ion-ios-navigate:before, .ion-ios-navigate-outline:before, .ion-ios-nutrition:before, .ion-ios-nutrition-outline:before, .ion-ios-paper:before, .ion-ios-paper-outline:before, .ion-ios-paperplane:before, .ion-ios-paperplane-outline:before, .ion-ios-partlysunny:before, .ion-ios-partlysunny-outline:before, .ion-ios-pause:before, .ion-ios-pause-outline:before, .ion-ios-paw:before, .ion-ios-paw-outline:before, .ion-ios-people:before, .ion-ios-people-outline:before, .ion-ios-person:before, .ion-ios-person-outline:before, .ion-ios-personadd:before, .ion-ios-personadd-outline:before, .ion-ios-photos:before, .ion-ios-photos-outline:before, .ion-ios-pie:before, .ion-ios-pie-outline:before, .ion-ios-pint:before, .ion-ios-pint-outline:before, .ion-ios-play:before, .ion-ios-play-outline:before, .ion-ios-plus:before, .ion-ios-plus-empty:before, .ion-ios-plus-outline:before, .ion-ios-pricetag:before, .ion-ios-pricetag-outline:before, .ion-ios-pricetags:before, .ion-ios-pricetags-outline:before, .ion-ios-printer:before, .ion-ios-printer-outline:before, .ion-ios-pulse:before, .ion-ios-pulse-strong:before, .ion-ios-rainy:before, .ion-ios-rainy-outline:before, .ion-ios-recording:before, .ion-ios-recording-outline:before, .ion-ios-redo:before, .ion-ios-redo-outline:before, .ion-ios-refresh:before, .ion-ios-refresh-empty:before, .ion-ios-refresh-outline:before, .ion-ios-reload:before, .ion-ios-reverse-camera:before, .ion-ios-reverse-camera-outline:before, .ion-ios-rewind:before, .ion-ios-rewind-outline:before, .ion-ios-rose:before, .ion-ios-rose-outline:before, .ion-ios-search:before, .ion-ios-search-strong:before, .ion-ios-settings:before, .ion-ios-settings-strong:before, .ion-ios-shuffle:before, .ion-ios-shuffle-strong:before, .ion-ios-skipbackward:before, .ion-ios-skipbackward-outline:before, .ion-ios-skipforward:before, .ion-ios-skipforward-outline:before, .ion-ios-snowy:before, .ion-ios-speedometer:before, .ion-ios-speedometer-outline:before, .ion-ios-star:before, .ion-ios-star-half:before, .ion-ios-star-outline:before, .ion-ios-stopwatch:before, .ion-ios-stopwatch-outline:before, .ion-ios-sunny:before, .ion-ios-sunny-outline:before, .ion-ios-telephone:before, .ion-ios-telephone-outline:before, .ion-ios-tennisball:before, .ion-ios-tennisball-outline:before, .ion-ios-thunderstorm:before, .ion-ios-thunderstorm-outline:before, .ion-ios-time:before, .ion-ios-time-outline:before, .ion-ios-timer:before, .ion-ios-timer-outline:before, .ion-ios-toggle:before, .ion-ios-toggle-outline:before, .ion-ios-trash:before, .ion-ios-trash-outline:before, .ion-ios-undo:before, .ion-ios-undo-outline:before, .ion-ios-unlocked:before, .ion-ios-unlocked-outline:before, .ion-ios-upload:before, .ion-ios-upload-outline:before, .ion-ios-videocam:before, .ion-ios-videocam-outline:before, .ion-ios-volume-high:before, .ion-ios-volume-low:before, .ion-ios-wineglass:before, .ion-ios-wineglass-outline:before, .ion-ios-world:before, .ion-ios-world-outline:before, .ion-ipad:before, .ion-iphone:before, .ion-ipod:before, .ion-jet:before, .ion-key:before, .ion-knife:before, .ion-laptop:before, .ion-leaf:before, .ion-levels:before, .ion-lightbulb:before, .ion-link:before, .ion-load-a:before, .ion-load-b:before, .ion-load-c:before, .ion-load-d:before, .ion-location:before, .ion-lock-combination:before, .ion-locked:before, .ion-log-in:before, .ion-log-out:before, .ion-loop:before, .ion-magnet:before, .ion-male:before, .ion-man:before, .ion-map:before, .ion-medkit:before, .ion-merge:before, .ion-mic-a:before, .ion-mic-b:before, .ion-mic-c:before, .ion-minus:before, .ion-minus-circled:before, .ion-minus-round:before, .ion-model-s:before, .ion-monitor:before, .ion-more:before, .ion-mouse:before, .ion-music-note:before, .ion-navicon:before, .ion-navicon-round:before, .ion-navigate:before, .ion-network:before, .ion-no-smoking:before, .ion-nuclear:before, .ion-outlet:before, .ion-paintbrush:before, .ion-paintbucket:before, .ion-paper-airplane:before, .ion-paperclip:before, .ion-pause:before, .ion-person:before, .ion-person-add:before, .ion-person-stalker:before, .ion-pie-graph:before, .ion-pin:before, .ion-pinpoint:before, .ion-pizza:before, .ion-plane:before, .ion-planet:before, .ion-play:before, .ion-playstation:before, .ion-plus:before, .ion-plus-circled:before, .ion-plus-round:before, .ion-podium:before, .ion-pound:before, .ion-power:before, .ion-pricetag:before, .ion-pricetags:before, .ion-printer:before, .ion-pull-request:before, .ion-qr-scanner:before, .ion-quote:before, .ion-radio-waves:before, .ion-record:before, .ion-refresh:before, .ion-reply:before, .ion-reply-all:before, .ion-ribbon-a:before, .ion-ribbon-b:before, .ion-sad:before, .ion-sad-outline:before, .ion-scissors:before, .ion-search:before, .ion-settings:before, .ion-share:before, .ion-shuffle:before, .ion-skip-backward:before, .ion-skip-forward:before, .ion-social-android:before, .ion-social-android-outline:before, .ion-social-angular:before, .ion-social-angular-outline:before, .ion-social-apple:before, .ion-social-apple-outline:before, .ion-social-bitcoin:before, .ion-social-bitcoin-outline:before, .ion-social-buffer:before, .ion-social-buffer-outline:before, .ion-social-chrome:before, .ion-social-chrome-outline:before, .ion-social-codepen:before, .ion-social-codepen-outline:before, .ion-social-css3:before, .ion-social-css3-outline:before, .ion-social-designernews:before, .ion-social-designernews-outline:before, .ion-social-dribbble:before, .ion-social-dribbble-outline:before, .ion-social-dropbox:before, .ion-social-dropbox-outline:before, .ion-social-euro:before, .ion-social-euro-outline:before, .ion-social-facebook:before, .ion-social-facebook-outline:before, .ion-social-foursquare:before, .ion-social-foursquare-outline:before, .ion-social-freebsd-devil:before, .ion-social-github:before, .ion-social-github-outline:before, .ion-social-google:before, .ion-social-google-outline:before, .ion-social-googleplus:before, .ion-social-googleplus-outline:before, .ion-social-hackernews:before, .ion-social-hackernews-outline:before, .ion-social-html5:before, .ion-social-html5-outline:before, .ion-social-instagram:before, .ion-social-instagram-outline:before, .ion-social-javascript:before, .ion-social-javascript-outline:before, .ion-social-linkedin:before, .ion-social-linkedin-outline:before, .ion-social-markdown:before, .ion-social-nodejs:before, .ion-social-octocat:before, .ion-social-pinterest:before, .ion-social-pinterest-outline:before, .ion-social-python:before, .ion-social-reddit:before, .ion-social-reddit-outline:before, .ion-social-rss:before, .ion-social-rss-outline:before, .ion-social-sass:before, .ion-social-skype:before, .ion-social-skype-outline:before, .ion-social-snapchat:before, .ion-social-snapchat-outline:before, .ion-social-tumblr:before, .ion-social-tumblr-outline:before, .ion-social-tux:before, .ion-social-twitch:before, .ion-social-twitch-outline:before, .ion-social-twitter:before, .ion-social-twitter-outline:before, .ion-social-usd:before, .ion-social-usd-outline:before, .ion-social-vimeo:before, .ion-social-vimeo-outline:before, .ion-social-whatsapp:before, .ion-social-whatsapp-outline:before, .ion-social-windows:before, .ion-social-windows-outline:before, .ion-social-wordpress:before, .ion-social-wordpress-outline:before, .ion-social-yahoo:before, .ion-social-yahoo-outline:before, .ion-social-yen:before, .ion-social-yen-outline:before, .ion-social-youtube:before, .ion-social-youtube-outline:before, .ion-soup-can:before, .ion-soup-can-outline:before, .ion-speakerphone:before, .ion-speedometer:before, .ion-spoon:before, .ion-star:before, .ion-stats-bars:before, .ion-steam:before, .ion-stop:before, .ion-thermometer:before, .ion-thumbsdown:before, .ion-thumbsup:before, .ion-toggle:before, .ion-toggle-filled:before, .ion-transgender:before, .ion-trash-a:before, .ion-trash-b:before, .ion-trophy:before, .ion-tshirt:before, .ion-tshirt-outline:before, .ion-umbrella:before, .ion-university:before, .ion-unlocked:before, .ion-upload:before, .ion-usb:before, .ion-videocamera:before, .ion-volume-high:before, .ion-volume-low:before, .ion-volume-medium:before, .ion-volume-mute:before, .ion-wand:before, .ion-waterdrop:before, .ion-wifi:before, .ion-wineglass:before, .ion-woman:before, .ion-wrench:before, .ion-xbox:before { display: inline-block; font-family: &quot;Ionicons&quot;; speak: none; font-style: normal; font-weight: normal; font-variant: normal; text-transform: none; text-rendering: auto; line-height: 1; -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; }
.ion-alert:before { content: &quot;\f101&quot;; }
.ion-alert-circled:before { content: &quot;\f100&quot;; }
.ion-android-add:before { content: &quot;\f2c7&quot;; }
.ion-android-add-circle:before { content: &quot;\f359&quot;; }
.ion-android-alarm-clock:before { content: &quot;\f35a&quot;; }
.ion-android-alert:before { content: &quot;\f35b&quot;; }
.ion-android-apps:before { content: &quot;\f35c&quot;; }
.ion-android-archive:before { content: &quot;\f2c9&quot;; }
.ion-android-arrow-back:before { content: &quot;\f2ca&quot;; }
.ion-android-arrow-down:before { content: &quot;\f35d&quot;; }
.ion-android-arrow-dropdown:before { content: &quot;\f35f&quot;; }
.ion-android-arrow-dropdown-circle:before { content: &quot;\f35e&quot;; }
.ion-android-arrow-dropleft:before { content: &quot;\f361&quot;; }
.ion-android-arrow-dropleft-circle:before { content: &quot;\f360&quot;; }
.ion-android-arrow-dropright:before { content: &quot;\f363&quot;; }
.ion-android-arrow-dropright-circle:before { content: &quot;\f362&quot;; }
.ion-android-arrow-dropup:before { content: &quot;\f365&quot;; }
.ion-android-arrow-dropup-circle:before { content: &quot;\f364&quot;; }
.ion-android-arrow-forward:before { content: &quot;\f30f&quot;; }
.ion-android-arrow-up:before { content: &quot;\f366&quot;; }
.ion-android-attach:before { content: &quot;\f367&quot;; }
.ion-android-bar:before { content: &quot;\f368&quot;; }
.ion-android-bicycle:before { content: &quot;\f369&quot;; }
.ion-android-boat:before { content: &quot;\f36a&quot;; }
.ion-android-bookmark:before { content: &quot;\f36b&quot;; }
.ion-android-bulb:before { content: &quot;\f36c&quot;; }
.ion-android-bus:before { content: &quot;\f36d&quot;; }
.ion-android-calendar:before { content: &quot;\f2d1&quot;; }
.ion-android-call:before { content: &quot;\f2d2&quot;; }
.ion-android-camera:before { content: &quot;\f2d3&quot;; }
.ion-android-cancel:before { content: &quot;\f36e&quot;; }
.ion-android-car:before { content: &quot;\f36f&quot;; }
.ion-android-cart:before { content: &quot;\f370&quot;; }
.ion-android-chat:before { content: &quot;\f2d4&quot;; }
.ion-android-checkbox:before { content: &quot;\f374&quot;; }
.ion-android-checkbox-blank:before { content: &quot;\f371&quot;; }
.ion-android-checkbox-outline:before { content: &quot;\f373&quot;; }
.ion-android-checkbox-outline-blank:before { content: &quot;\f372&quot;; }
.ion-android-checkmark-circle:before { content: &quot;\f375&quot;; }
.ion-android-clipboard:before { content: &quot;\f376&quot;; }
.ion-android-close:before { content: &quot;\f2d7&quot;; }
.ion-android-cloud:before { content: &quot;\f37a&quot;; }
.ion-android-cloud-circle:before { content: &quot;\f377&quot;; }
.ion-android-cloud-done:before { content: &quot;\f378&quot;; }
.ion-android-cloud-outline:before { content: &quot;\f379&quot;; }
.ion-android-color-palette:before { content: &quot;\f37b&quot;; }
.ion-android-compass:before { content: &quot;\f37c&quot;; }
.ion-android-contact:before { content: &quot;\f2d8&quot;; }
.ion-android-contacts:before { content: &quot;\f2d9&quot;; }
.ion-android-contract:before { content: &quot;\f37d&quot;; }
.ion-android-create:before { content: &quot;\f37e&quot;; }
.ion-android-delete:before { content: &quot;\f37f&quot;; }
.ion-android-desktop:before { content: &quot;\f380&quot;; }
.ion-android-document:before { content: &quot;\f381&quot;; }
.ion-android-done:before { content: &quot;\f383&quot;; }
.ion-android-done-all:before { content: &quot;\f382&quot;; }
.ion-android-download:before { content: &quot;\f2dd&quot;; }
.ion-android-drafts:before { content: &quot;\f384&quot;; }
.ion-android-exit:before { content: &quot;\f385&quot;; }
.ion-android-expand:before { content: &quot;\f386&quot;; }
.ion-android-favorite:before { content: &quot;\f388&quot;; }
.ion-android-favorite-outline:before { content: &quot;\f387&quot;; }
.ion-android-film:before { content: &quot;\f389&quot;; }
.ion-android-folder:before { content: &quot;\f2e0&quot;; }
.ion-android-folder-open:before { content: &quot;\f38a&quot;; }
.ion-android-funnel:before { content: &quot;\f38b&quot;; }
.ion-android-globe:before { content: &quot;\f38c&quot;; }
.ion-android-hand:before { content: &quot;\f2e3&quot;; }
.ion-android-hangout:before { content: &quot;\f38d&quot;; }
.ion-android-happy:before { content: &quot;\f38e&quot;; }
.ion-android-home:before { content: &quot;\f38f&quot;; }
.ion-android-image:before { content: &quot;\f2e4&quot;; }
.ion-android-laptop:before { content: &quot;\f390&quot;; }
.ion-android-list:before { content: &quot;\f391&quot;; }
.ion-android-locate:before { content: &quot;\f2e9&quot;; }
.ion-android-lock:before { content: &quot;\f392&quot;; }
.ion-android-mail:before { content: &quot;\f2eb&quot;; }
.ion-android-map:before { content: &quot;\f393&quot;; }
.ion-android-menu:before { content: &quot;\f394&quot;; }
.ion-android-microphone:before { content: &quot;\f2ec&quot;; }
.ion-android-microphone-off:before { content: &quot;\f395&quot;; }
.ion-android-more-horizontal:before { content: &quot;\f396&quot;; }
.ion-android-more-vertical:before { content: &quot;\f397&quot;; }
.ion-android-navigate:before { content: &quot;\f398&quot;; }
.ion-android-notifications:before { content: &quot;\f39b&quot;; }
.ion-android-notifications-none:before { content: &quot;\f399&quot;; }
.ion-android-notifications-off:before { content: &quot;\f39a&quot;; }
.ion-android-open:before { content: &quot;\f39c&quot;; }
.ion-android-options:before { content: &quot;\f39d&quot;; }
.ion-android-people:before { content: &quot;\f39e&quot;; }
.ion-android-person:before { content: &quot;\f3a0&quot;; }
.ion-android-person-add:before { content: &quot;\f39f&quot;; }
.ion-android-phone-landscape:before { content: &quot;\f3a1&quot;; }
.ion-android-phone-portrait:before { content: &quot;\f3a2&quot;; }
.ion-android-pin:before { content: &quot;\f3a3&quot;; }
.ion-android-plane:before { content: &quot;\f3a4&quot;; }
.ion-android-playstore:before { content: &quot;\f2f0&quot;; }
.ion-android-print:before { content: &quot;\f3a5&quot;; }
.ion-android-radio-button-off:before { content: &quot;\f3a6&quot;; }
.ion-android-radio-button-on:before { content: &quot;\f3a7&quot;; }
.ion-android-refresh:before { content: &quot;\f3a8&quot;; }
.ion-android-remove:before { content: &quot;\f2f4&quot;; }
.ion-android-remove-circle:before { content: &quot;\f3a9&quot;; }
.ion-android-restaurant:before { content: &quot;\f3aa&quot;; }
.ion-android-sad:before { content: &quot;\f3ab&quot;; }
.ion-android-search:before { content: &quot;\f2f5&quot;; }
.ion-android-send:before { content: &quot;\f2f6&quot;; }
.ion-android-settings:before { content: &quot;\f2f7&quot;; }
.ion-android-share:before { content: &quot;\f2f8&quot;; }
.ion-android-share-alt:before { content: &quot;\f3ac&quot;; }
.ion-android-star:before { content: &quot;\f2fc&quot;; }
.ion-android-star-half:before { content: &quot;\f3ad&quot;; }
.ion-android-star-outline:before { content: &quot;\f3ae&quot;; }
.ion-android-stopwatch:before { content: &quot;\f2fd&quot;; }
.ion-android-subway:before { content: &quot;\f3af&quot;; }
.ion-android-sunny:before { content: &quot;\f3b0&quot;; }
.ion-android-sync:before { content: &quot;\f3b1&quot;; }
.ion-android-textsms:before { content: &quot;\f3b2&quot;; }
.ion-android-time:before { content: &quot;\f3b3&quot;; }
.ion-android-train:before { content: &quot;\f3b4&quot;; }
.ion-android-unlock:before { content: &quot;\f3b5&quot;; }
.ion-android-upload:before { content: &quot;\f3b6&quot;; }
.ion-android-volume-down:before { content: &quot;\f3b7&quot;; }
.ion-android-volume-mute:before { content: &quot;\f3b8&quot;; }
.ion-android-volume-off:before { content: &quot;\f3b9&quot;; }
.ion-android-volume-up:before { content: &quot;\f3ba&quot;; }
.ion-android-walk:before { content: &quot;\f3bb&quot;; }
.ion-android-warning:before { content: &quot;\f3bc&quot;; }
.ion-android-watch:before { content: &quot;\f3bd&quot;; }
.ion-android-wifi:before { content: &quot;\f305&quot;; }
.ion-aperture:before { content: &quot;\f313&quot;; }
.ion-archive:before { content: &quot;\f102&quot;; }
.ion-arrow-down-a:before { content: &quot;\f103&quot;; }
.ion-arrow-down-b:before { content: &quot;\f104&quot;; }
.ion-arrow-down-c:before { content: &quot;\f105&quot;; }
.ion-arrow-expand:before { content: &quot;\f25e&quot;; }
.ion-arrow-graph-down-left:before { content: &quot;\f25f&quot;; }
.ion-arrow-graph-down-right:before { content: &quot;\f260&quot;; }
.ion-arrow-graph-up-left:before { content: &quot;\f261&quot;; }
.ion-arrow-graph-up-right:before { content: &quot;\f262&quot;; }
.ion-arrow-left-a:before { content: &quot;\f106&quot;; }
.ion-arrow-left-b:before { content: &quot;\f107&quot;; }
.ion-arrow-left-c:before { content: &quot;\f108&quot;; }
.ion-arrow-move:before { content: &quot;\f263&quot;; }
.ion-arrow-resize:before { content: &quot;\f264&quot;; }
.ion-arrow-return-left:before { content: &quot;\f265&quot;; }
.ion-arrow-return-right:before { content: &quot;\f266&quot;; }
.ion-arrow-right-a:before { content: &quot;\f109&quot;; }
.ion-arrow-right-b:before { content: &quot;\f10a&quot;; }
.ion-arrow-right-c:before { content: &quot;\f10b&quot;; }
.ion-arrow-shrink:before { content: &quot;\f267&quot;; }
.ion-arrow-swap:before { content: &quot;\f268&quot;; }
.ion-arrow-up-a:before { content: &quot;\f10c&quot;; }
.ion-arrow-up-b:before { content: &quot;\f10d&quot;; }
.ion-arrow-up-c:before { content: &quot;\f10e&quot;; }
.ion-asterisk:before { content: &quot;\f314&quot;; }
.ion-at:before { content: &quot;\f10f&quot;; }
.ion-backspace:before { content: &quot;\f3bf&quot;; }
.ion-backspace-outline:before { content: &quot;\f3be&quot;; }
.ion-bag:before { content: &quot;\f110&quot;; }
.ion-battery-charging:before { content: &quot;\f111&quot;; }
.ion-battery-empty:before { content: &quot;\f112&quot;; }
.ion-battery-full:before { content: &quot;\f113&quot;; }
.ion-battery-half:before { content: &quot;\f114&quot;; }
.ion-battery-low:before { content: &quot;\f115&quot;; }
.ion-beaker:before { content: &quot;\f269&quot;; }
.ion-beer:before { content: &quot;\f26a&quot;; }
.ion-bluetooth:before { content: &quot;\f116&quot;; }
.ion-bonfire:before { content: &quot;\f315&quot;; }
.ion-bookmark:before { content: &quot;\f26b&quot;; }
.ion-bowtie:before { content: &quot;\f3c0&quot;; }
.ion-briefcase:before { content: &quot;\f26c&quot;; }
.ion-bug:before { content: &quot;\f2be&quot;; }
.ion-calculator:before { content: &quot;\f26d&quot;; }
.ion-calendar:before { content: &quot;\f117&quot;; }
.ion-camera:before { content: &quot;\f118&quot;; }
.ion-card:before { content: &quot;\f119&quot;; }
.ion-cash:before { content: &quot;\f316&quot;; }
.ion-chatbox:before { content: &quot;\f11b&quot;; }
.ion-chatbox-working:before { content: &quot;\f11a&quot;; }
.ion-chatboxes:before { content: &quot;\f11c&quot;; }
.ion-chatbubble:before { content: &quot;\f11e&quot;; }
.ion-chatbubble-working:before { content: &quot;\f11d&quot;; }
.ion-chatbubbles:before { content: &quot;\f11f&quot;; }
.ion-checkmark:before { content: &quot;\f122&quot;; }
.ion-checkmark-circled:before { content: &quot;\f120&quot;; }
.ion-checkmark-round:before { content: &quot;\f121&quot;; }
.ion-chevron-down:before { content: &quot;\f123&quot;; }
.ion-chevron-left:before { content: &quot;\f124&quot;; }
.ion-chevron-right:before { content: &quot;\f125&quot;; }
.ion-chevron-up:before { content: &quot;\f126&quot;; }
.ion-clipboard:before { content: &quot;\f127&quot;; }
.ion-clock:before { content: &quot;\f26e&quot;; }
.ion-close:before { content: &quot;\f12a&quot;; }
.ion-close-circled:before { content: &quot;\f128&quot;; }
.ion-close-round:before { content: &quot;\f129&quot;; }
.ion-closed-captioning:before { content: &quot;\f317&quot;; }
.ion-cloud:before { content: &quot;\f12b&quot;; }
.ion-code:before { content: &quot;\f271&quot;; }
.ion-code-download:before { content: &quot;\f26f&quot;; }
.ion-code-working:before { content: &quot;\f270&quot;; }
.ion-coffee:before { content: &quot;\f272&quot;; }
.ion-compass:before { content: &quot;\f273&quot;; }
.ion-compose:before { content: &quot;\f12c&quot;; }
.ion-connection-bars:before { content: &quot;\f274&quot;; }
.ion-contrast:before { content: &quot;\f275&quot;; }
.ion-crop:before { content: &quot;\f3c1&quot;; }
.ion-cube:before { content: &quot;\f318&quot;; }
.ion-disc:before { content: &quot;\f12d&quot;; }
.ion-document:before { content: &quot;\f12f&quot;; }
.ion-document-text:before { content: &quot;\f12e&quot;; }
.ion-drag:before { content: &quot;\f130&quot;; }
.ion-earth:before { content: &quot;\f276&quot;; }
.ion-easel:before { content: &quot;\f3c2&quot;; }
.ion-edit:before { content: &quot;\f2bf&quot;; }
.ion-egg:before { content: &quot;\f277&quot;; }
.ion-eject:before { content: &quot;\f131&quot;; }
.ion-email:before { content: &quot;\f132&quot;; }
.ion-email-unread:before { content: &quot;\f3c3&quot;; }
.ion-erlenmeyer-flask:before { content: &quot;\f3c5&quot;; }
.ion-erlenmeyer-flask-bubbles:before { content: &quot;\f3c4&quot;; }
.ion-eye:before { content: &quot;\f133&quot;; }
.ion-eye-disabled:before { content: &quot;\f306&quot;; }
.ion-female:before { content: &quot;\f278&quot;; }
.ion-filing:before { content: &quot;\f134&quot;; }
.ion-film-marker:before { content: &quot;\f135&quot;; }
.ion-fireball:before { content: &quot;\f319&quot;; }
.ion-flag:before { content: &quot;\f279&quot;; }
.ion-flame:before { content: &quot;\f31a&quot;; }
.ion-flash:before { content: &quot;\f137&quot;; }
.ion-flash-off:before { content: &quot;\f136&quot;; }
.ion-folder:before { content: &quot;\f139&quot;; }
.ion-fork:before { content: &quot;\f27a&quot;; }
.ion-fork-repo:before { content: &quot;\f2c0&quot;; }
.ion-forward:before { content: &quot;\f13a&quot;; }
.ion-funnel:before { content: &quot;\f31b&quot;; }
.ion-gear-a:before { content: &quot;\f13d&quot;; }
.ion-gear-b:before { content: &quot;\f13e&quot;; }
.ion-grid:before { content: &quot;\f13f&quot;; }
.ion-hammer:before { content: &quot;\f27b&quot;; }
.ion-happy:before { content: &quot;\f31c&quot;; }
.ion-happy-outline:before { content: &quot;\f3c6&quot;; }
.ion-headphone:before { content: &quot;\f140&quot;; }
.ion-heart:before { content: &quot;\f141&quot;; }
.ion-heart-broken:before { content: &quot;\f31d&quot;; }
.ion-help:before { content: &quot;\f143&quot;; }
.ion-help-buoy:before { content: &quot;\f27c&quot;; }
.ion-help-circled:before { content: &quot;\f142&quot;; }
.ion-home:before { content: &quot;\f144&quot;; }
.ion-icecream:before { content: &quot;\f27d&quot;; }
.ion-image:before { content: &quot;\f147&quot;; }
.ion-images:before { content: &quot;\f148&quot;; }
.ion-information:before { content: &quot;\f14a&quot;; }
.ion-information-circled:before { content: &quot;\f149&quot;; }
.ion-ionic:before { content: &quot;\f14b&quot;; }
.ion-ios-alarm:before { content: &quot;\f3c8&quot;; }
.ion-ios-alarm-outline:before { content: &quot;\f3c7&quot;; }
.ion-ios-albums:before { content: &quot;\f3ca&quot;; }
.ion-ios-albums-outline:before { content: &quot;\f3c9&quot;; }
.ion-ios-americanfootball:before { content: &quot;\f3cc&quot;; }
.ion-ios-americanfootball-outline:before { content: &quot;\f3cb&quot;; }
.ion-ios-analytics:before { content: &quot;\f3ce&quot;; }
.ion-ios-analytics-outline:before { content: &quot;\f3cd&quot;; }
.ion-ios-arrow-back:before { content: &quot;\f3cf&quot;; }
.ion-ios-arrow-down:before { content: &quot;\f3d0&quot;; }
.ion-ios-arrow-forward:before { content: &quot;\f3d1&quot;; }
.ion-ios-arrow-left:before { content: &quot;\f3d2&quot;; }
.ion-ios-arrow-right:before { content: &quot;\f3d3&quot;; }
.ion-ios-arrow-thin-down:before { content: &quot;\f3d4&quot;; }
.ion-ios-arrow-thin-left:before { content: &quot;\f3d5&quot;; }
.ion-ios-arrow-thin-right:before { content: &quot;\f3d6&quot;; }
.ion-ios-arrow-thin-up:before { content: &quot;\f3d7&quot;; }
.ion-ios-arrow-up:before { content: &quot;\f3d8&quot;; }
.ion-ios-at:before { content: &quot;\f3da&quot;; }
.ion-ios-at-outline:before { content: &quot;\f3d9&quot;; }
.ion-ios-barcode:before { content: &quot;\f3dc&quot;; }
.ion-ios-barcode-outline:before { content: &quot;\f3db&quot;; }
.ion-ios-baseball:before { content: &quot;\f3de&quot;; }
.ion-ios-baseball-outline:before { content: &quot;\f3dd&quot;; }
.ion-ios-basketball:before { content: &quot;\f3e0&quot;; }
.ion-ios-basketball-outline:before { content: &quot;\f3df&quot;; }
.ion-ios-bell:before { content: &quot;\f3e2&quot;; }
.ion-ios-bell-outline:before { content: &quot;\f3e1&quot;; }
.ion-ios-body:before { content: &quot;\f3e4&quot;; }
.ion-ios-body-outline:before { content: &quot;\f3e3&quot;; }
.ion-ios-bolt:before { content: &quot;\f3e6&quot;; }
.ion-ios-bolt-outline:before { content: &quot;\f3e5&quot;; }
.ion-ios-book:before { content: &quot;\f3e8&quot;; }
.ion-ios-book-outline:before { content: &quot;\f3e7&quot;; }
.ion-ios-bookmarks:before { content: &quot;\f3ea&quot;; }
.ion-ios-bookmarks-outline:before { content: &quot;\f3e9&quot;; }
.ion-ios-box:before { content: &quot;\f3ec&quot;; }
.ion-ios-box-outline:before { content: &quot;\f3eb&quot;; }
.ion-ios-briefcase:before { content: &quot;\f3ee&quot;; }
.ion-ios-briefcase-outline:before { content: &quot;\f3ed&quot;; }
.ion-ios-browsers:before { content: &quot;\f3f0&quot;; }
.ion-ios-browsers-outline:before { content: &quot;\f3ef&quot;; }
.ion-ios-calculator:before { content: &quot;\f3f2&quot;; }
.ion-ios-calculator-outline:before { content: &quot;\f3f1&quot;; }
.ion-ios-calendar:before { content: &quot;\f3f4&quot;; }
.ion-ios-calendar-outline:before { content: &quot;\f3f3&quot;; }
.ion-ios-camera:before { content: &quot;\f3f6&quot;; }
.ion-ios-camera-outline:before { content: &quot;\f3f5&quot;; }
.ion-ios-cart:before { content: &quot;\f3f8&quot;; }
.ion-ios-cart-outline:before { content: &quot;\f3f7&quot;; }
.ion-ios-chatboxes:before { content: &quot;\f3fa&quot;; }
.ion-ios-chatboxes-outline:before { content: &quot;\f3f9&quot;; }
.ion-ios-chatbubble:before { content: &quot;\f3fc&quot;; }
.ion-ios-chatbubble-outline:before { content: &quot;\f3fb&quot;; }
.ion-ios-checkmark:before { content: &quot;\f3ff&quot;; }
.ion-ios-checkmark-empty:before { content: &quot;\f3fd&quot;; }
.ion-ios-checkmark-outline:before { content: &quot;\f3fe&quot;; }
.ion-ios-circle-filled:before { content: &quot;\f400&quot;; }
.ion-ios-circle-outline:before { content: &quot;\f401&quot;; }
.ion-ios-clock:before { content: &quot;\f403&quot;; }
.ion-ios-clock-outline:before { content: &quot;\f402&quot;; }
.ion-ios-close:before { content: &quot;\f406&quot;; }
.ion-ios-close-empty:before { content: &quot;\f404&quot;; }
.ion-ios-close-outline:before { content: &quot;\f405&quot;; }
.ion-ios-cloud:before { content: &quot;\f40c&quot;; }
.ion-ios-cloud-download:before { content: &quot;\f408&quot;; }
.ion-ios-cloud-download-outline:before { content: &quot;\f407&quot;; }
.ion-ios-cloud-outline:before { content: &quot;\f409&quot;; }
.ion-ios-cloud-upload:before { content: &quot;\f40b&quot;; }
.ion-ios-cloud-upload-outline:before { content: &quot;\f40a&quot;; }
.ion-ios-cloudy:before { content: &quot;\f410&quot;; }
.ion-ios-cloudy-night:before { content: &quot;\f40e&quot;; }
.ion-ios-cloudy-night-outline:before { content: &quot;\f40d&quot;; }
.ion-ios-cloudy-outline:before { content: &quot;\f40f&quot;; }
.ion-ios-cog:before { content: &quot;\f412&quot;; }
.ion-ios-cog-outline:before { content: &quot;\f411&quot;; }
.ion-ios-color-filter:before { content: &quot;\f414&quot;; }
.ion-ios-color-filter-outline:before { content: &quot;\f413&quot;; }
.ion-ios-color-wand:before { content: &quot;\f416&quot;; }
.ion-ios-color-wand-outline:before { content: &quot;\f415&quot;; }
.ion-ios-compose:before { content: &quot;\f418&quot;; }
.ion-ios-compose-outline:before { content: &quot;\f417&quot;; }
.ion-ios-contact:before { content: &quot;\f41a&quot;; }
.ion-ios-contact-outline:before { content: &quot;\f419&quot;; }
.ion-ios-copy:before { content: &quot;\f41c&quot;; }
.ion-ios-copy-outline:before { content: &quot;\f41b&quot;; }
.ion-ios-crop:before { content: &quot;\f41e&quot;; }
.ion-ios-crop-strong:before { content: &quot;\f41d&quot;; }
.ion-ios-download:before { content: &quot;\f420&quot;; }
.ion-ios-download-outline:before { content: &quot;\f41f&quot;; }
.ion-ios-drag:before { content: &quot;\f421&quot;; }
.ion-ios-email:before { content: &quot;\f423&quot;; }
.ion-ios-email-outline:before { content: &quot;\f422&quot;; }
.ion-ios-eye:before { content: &quot;\f425&quot;; }
.ion-ios-eye-outline:before { content: &quot;\f424&quot;; }
.ion-ios-fastforward:before { content: &quot;\f427&quot;; }
.ion-ios-fastforward-outline:before { content: &quot;\f426&quot;; }
.ion-ios-filing:before { content: &quot;\f429&quot;; }
.ion-ios-filing-outline:before { content: &quot;\f428&quot;; }
.ion-ios-film:before { content: &quot;\f42b&quot;; }
.ion-ios-film-outline:before { content: &quot;\f42a&quot;; }
.ion-ios-flag:before { content: &quot;\f42d&quot;; }
.ion-ios-flag-outline:before { content: &quot;\f42c&quot;; }
.ion-ios-flame:before { content: &quot;\f42f&quot;; }
.ion-ios-flame-outline:before { content: &quot;\f42e&quot;; }
.ion-ios-flask:before { content: &quot;\f431&quot;; }
.ion-ios-flask-outline:before { content: &quot;\f430&quot;; }
.ion-ios-flower:before { content: &quot;\f433&quot;; }
.ion-ios-flower-outline:before { content: &quot;\f432&quot;; }
.ion-ios-folder:before { content: &quot;\f435&quot;; }
.ion-ios-folder-outline:before { content: &quot;\f434&quot;; }
.ion-ios-football:before { content: &quot;\f437&quot;; }
.ion-ios-football-outline:before { content: &quot;\f436&quot;; }
.ion-ios-game-controller-a:before { content: &quot;\f439&quot;; }
.ion-ios-game-controller-a-outline:before { content: &quot;\f438&quot;; }
.ion-ios-game-controller-b:before { content: &quot;\f43b&quot;; }
.ion-ios-game-controller-b-outline:before { content: &quot;\f43a&quot;; }
.ion-ios-gear:before { content: &quot;\f43d&quot;; }
.ion-ios-gear-outline:before { content: &quot;\f43c&quot;; }
.ion-ios-glasses:before { content: &quot;\f43f&quot;; }
.ion-ios-glasses-outline:before { content: &quot;\f43e&quot;; }
.ion-ios-grid-view:before { content: &quot;\f441&quot;; }
.ion-ios-grid-view-outline:before { content: &quot;\f440&quot;; }
.ion-ios-heart:before { content: &quot;\f443&quot;; }
.ion-ios-heart-outline:before { content: &quot;\f442&quot;; }
.ion-ios-help:before { content: &quot;\f446&quot;; }
.ion-ios-help-empty:before { content: &quot;\f444&quot;; }
.ion-ios-help-outline:before { content: &quot;\f445&quot;; }
.ion-ios-home:before { content: &quot;\f448&quot;; }
.ion-ios-home-outline:before { content: &quot;\f447&quot;; }
.ion-ios-infinite:before { content: &quot;\f44a&quot;; }
.ion-ios-infinite-outline:before { content: &quot;\f449&quot;; }
.ion-ios-information:before { content: &quot;\f44d&quot;; }
.ion-ios-information-empty:before { content: &quot;\f44b&quot;; }
.ion-ios-information-outline:before { content: &quot;\f44c&quot;; }
.ion-ios-ionic-outline:before { content: &quot;\f44e&quot;; }
.ion-ios-keypad:before { content: &quot;\f450&quot;; }
.ion-ios-keypad-outline:before { content: &quot;\f44f&quot;; }
.ion-ios-lightbulb:before { content: &quot;\f452&quot;; }
.ion-ios-lightbulb-outline:before { content: &quot;\f451&quot;; }
.ion-ios-list:before { content: &quot;\f454&quot;; }
.ion-ios-list-outline:before { content: &quot;\f453&quot;; }
.ion-ios-location:before { content: &quot;\f456&quot;; }
.ion-ios-location-outline:before { content: &quot;\f455&quot;; }
.ion-ios-locked:before { content: &quot;\f458&quot;; }
.ion-ios-locked-outline:before { content: &quot;\f457&quot;; }
.ion-ios-loop:before { content: &quot;\f45a&quot;; }
.ion-ios-loop-strong:before { content: &quot;\f459&quot;; }
.ion-ios-medical:before { content: &quot;\f45c&quot;; }
.ion-ios-medical-outline:before { content: &quot;\f45b&quot;; }
.ion-ios-medkit:before { content: &quot;\f45e&quot;; }
.ion-ios-medkit-outline:before { content: &quot;\f45d&quot;; }
.ion-ios-mic:before { content: &quot;\f461&quot;; }
.ion-ios-mic-off:before { content: &quot;\f45f&quot;; }
.ion-ios-mic-outline:before { content: &quot;\f460&quot;; }
.ion-ios-minus:before { content: &quot;\f464&quot;; }
.ion-ios-minus-empty:before { content: &quot;\f462&quot;; }
.ion-ios-minus-outline:before { content: &quot;\f463&quot;; }
.ion-ios-monitor:before { content: &quot;\f466&quot;; }
.ion-ios-monitor-outline:before { content: &quot;\f465&quot;; }
.ion-ios-moon:before { content: &quot;\f468&quot;; }
.ion-ios-moon-outline:before { content: &quot;\f467&quot;; }
.ion-ios-more:before { content: &quot;\f46a&quot;; }
.ion-ios-more-outline:before { content: &quot;\f469&quot;; }
.ion-ios-musical-note:before { content: &quot;\f46b&quot;; }
.ion-ios-musical-notes:before { content: &quot;\f46c&quot;; }
.ion-ios-navigate:before { content: &quot;\f46e&quot;; }
.ion-ios-navigate-outline:before { content: &quot;\f46d&quot;; }
.ion-ios-nutrition:before { content: &quot;\f470&quot;; }
.ion-ios-nutrition-outline:before { content: &quot;\f46f&quot;; }
.ion-ios-paper:before { content: &quot;\f472&quot;; }
.ion-ios-paper-outline:before { content: &quot;\f471&quot;; }
.ion-ios-paperplane:before { content: &quot;\f474&quot;; }
.ion-ios-paperplane-outline:before { content: &quot;\f473&quot;; }
.ion-ios-partlysunny:before { content: &quot;\f476&quot;; }
.ion-ios-partlysunny-outline:before { content: &quot;\f475&quot;; }
.ion-ios-pause:before { content: &quot;\f478&quot;; }
.ion-ios-pause-outline:before { content: &quot;\f477&quot;; }
.ion-ios-paw:before { content: &quot;\f47a&quot;; }
.ion-ios-paw-outline:before { content: &quot;\f479&quot;; }
.ion-ios-people:before { content: &quot;\f47c&quot;; }
.ion-ios-people-outline:before { content: &quot;\f47b&quot;; }
.ion-ios-person:before { content: &quot;\f47e&quot;; }
.ion-ios-person-outline:before { content: &quot;\f47d&quot;; }
.ion-ios-personadd:before { content: &quot;\f480&quot;; }
.ion-ios-personadd-outline:before { content: &quot;\f47f&quot;; }
.ion-ios-photos:before { content: &quot;\f482&quot;; }
.ion-ios-photos-outline:before { content: &quot;\f481&quot;; }
.ion-ios-pie:before { content: &quot;\f484&quot;; }
.ion-ios-pie-outline:before { content: &quot;\f483&quot;; }
.ion-ios-pint:before { content: &quot;\f486&quot;; }
.ion-ios-pint-outline:before { content: &quot;\f485&quot;; }
.ion-ios-play:before { content: &quot;\f488&quot;; }
.ion-ios-play-outline:before { content: &quot;\f487&quot;; }
.ion-ios-plus:before { content: &quot;\f48b&quot;; }
.ion-ios-plus-empty:before { content: &quot;\f489&quot;; }
.ion-ios-plus-outline:before { content: &quot;\f48a&quot;; }
.ion-ios-pricetag:before { content: &quot;\f48d&quot;; }
.ion-ios-pricetag-outline:before { content: &quot;\f48c&quot;; }
.ion-ios-pricetags:before { content: &quot;\f48f&quot;; }
.ion-ios-pricetags-outline:before { content: &quot;\f48e&quot;; }
.ion-ios-printer:before { content: &quot;\f491&quot;; }
.ion-ios-printer-outline:before { content: &quot;\f490&quot;; }
.ion-ios-pulse:before { content: &quot;\f493&quot;; }
.ion-ios-pulse-strong:before { content: &quot;\f492&quot;; }
.ion-ios-rainy:before { content: &quot;\f495&quot;; }
.ion-ios-rainy-outline:before { content: &quot;\f494&quot;; }
.ion-ios-recording:before { content: &quot;\f497&quot;; }
.ion-ios-recording-outline:before { content: &quot;\f496&quot;; }
.ion-ios-redo:before { content: &quot;\f499&quot;; }
.ion-ios-redo-outline:before { content: &quot;\f498&quot;; }
.ion-ios-refresh:before { content: &quot;\f49c&quot;; }
.ion-ios-refresh-empty:before { content: &quot;\f49a&quot;; }
.ion-ios-refresh-outline:before { content: &quot;\f49b&quot;; }
.ion-ios-reload:before { content: &quot;\f49d&quot;; }
.ion-ios-reverse-camera:before { content: &quot;\f49f&quot;; }
.ion-ios-reverse-camera-outline:before { content: &quot;\f49e&quot;; }
.ion-ios-rewind:before { content: &quot;\f4a1&quot;; }
.ion-ios-rewind-outline:before { content: &quot;\f4a0&quot;; }
.ion-ios-rose:before { content: &quot;\f4a3&quot;; }
.ion-ios-rose-outline:before { content: &quot;\f4a2&quot;; }
.ion-ios-search:before { content: &quot;\f4a5&quot;; }
.ion-ios-search-strong:before { content: &quot;\f4a4&quot;; }
.ion-ios-settings:before { content: &quot;\f4a7&quot;; }
.ion-ios-settings-strong:before { content: &quot;\f4a6&quot;; }
.ion-ios-shuffle:before { content: &quot;\f4a9&quot;; }
.ion-ios-shuffle-strong:before { content: &quot;\f4a8&quot;; }
.ion-ios-skipbackward:before { content: &quot;\f4ab&quot;; }
.ion-ios-skipbackward-outline:before { content: &quot;\f4aa&quot;; }
.ion-ios-skipforward:before { content: &quot;\f4ad&quot;; }
.ion-ios-skipforward-outline:before { content: &quot;\f4ac&quot;; }
.ion-ios-snowy:before { content: &quot;\f4ae&quot;; }
.ion-ios-speedometer:before { content: &quot;\f4b0&quot;; }
.ion-ios-speedometer-outline:before { content: &quot;\f4af&quot;; }
.ion-ios-star:before { content: &quot;\f4b3&quot;; }
.ion-ios-star-half:before { content: &quot;\f4b1&quot;; }
.ion-ios-star-outline:before { content: &quot;\f4b2&quot;; }
.ion-ios-stopwatch:before { content: &quot;\f4b5&quot;; }
.ion-ios-stopwatch-outline:before { content: &quot;\f4b4&quot;; }
.ion-ios-sunny:before { content: &quot;\f4b7&quot;; }
.ion-ios-sunny-outline:before { content: &quot;\f4b6&quot;; }
.ion-ios-telephone:before { content: &quot;\f4b9&quot;; }
.ion-ios-telephone-outline:before { content: &quot;\f4b8&quot;; }
.ion-ios-tennisball:before { content: &quot;\f4bb&quot;; }
.ion-ios-tennisball-outline:before { content: &quot;\f4ba&quot;; }
.ion-ios-thunderstorm:before { content: &quot;\f4bd&quot;; }
.ion-ios-thunderstorm-outline:before { content: &quot;\f4bc&quot;; }
.ion-ios-time:before { content: &quot;\f4bf&quot;; }
.ion-ios-time-outline:before { content: &quot;\f4be&quot;; }
.ion-ios-timer:before { content: &quot;\f4c1&quot;; }
.ion-ios-timer-outline:before { content: &quot;\f4c0&quot;; }
.ion-ios-toggle:before { content: &quot;\f4c3&quot;; }
.ion-ios-toggle-outline:before { content: &quot;\f4c2&quot;; }
.ion-ios-trash:before { content: &quot;\f4c5&quot;; }
.ion-ios-trash-outline:before { content: &quot;\f4c4&quot;; }
.ion-ios-undo:before { content: &quot;\f4c7&quot;; }
.ion-ios-undo-outline:before { content: &quot;\f4c6&quot;; }
.ion-ios-unlocked:before { content: &quot;\f4c9&quot;; }
.ion-ios-unlocked-outline:before { content: &quot;\f4c8&quot;; }
.ion-ios-upload:before { content: &quot;\f4cb&quot;; }
.ion-ios-upload-outline:before { content: &quot;\f4ca&quot;; }
.ion-ios-videocam:before { content: &quot;\f4cd&quot;; }
.ion-ios-videocam-outline:before { content: &quot;\f4cc&quot;; }
.ion-ios-volume-high:before { content: &quot;\f4ce&quot;; }
.ion-ios-volume-low:before { content: &quot;\f4cf&quot;; }
.ion-ios-wineglass:before { content: &quot;\f4d1&quot;; }
.ion-ios-wineglass-outline:before { content: &quot;\f4d0&quot;; }
.ion-ios-world:before { content: &quot;\f4d3&quot;; }
.ion-ios-world-outline:before { content: &quot;\f4d2&quot;; }
.ion-ipad:before { content: &quot;\f1f9&quot;; }
.ion-iphone:before { content: &quot;\f1fa&quot;; }
.ion-ipod:before { content: &quot;\f1fb&quot;; }
.ion-jet:before { content: &quot;\f295&quot;; }
.ion-key:before { content: &quot;\f296&quot;; }
.ion-knife:before { content: &quot;\f297&quot;; }
.ion-laptop:before { content: &quot;\f1fc&quot;; }
.ion-leaf:before { content: &quot;\f1fd&quot;; }
.ion-levels:before { content: &quot;\f298&quot;; }
.ion-lightbulb:before { content: &quot;\f299&quot;; }
.ion-link:before { content: &quot;\f1fe&quot;; }
.ion-load-a:before { content: &quot;\f29a&quot;; }
.ion-load-b:before { content: &quot;\f29b&quot;; }
.ion-load-c:before { content: &quot;\f29c&quot;; }
.ion-load-d:before { content: &quot;\f29d&quot;; }
.ion-location:before { content: &quot;\f1ff&quot;; }
.ion-lock-combination:before { content: &quot;\f4d4&quot;; }
.ion-locked:before { content: &quot;\f200&quot;; }
.ion-log-in:before { content: &quot;\f29e&quot;; }
.ion-log-out:before { content: &quot;\f29f&quot;; }
.ion-loop:before { content: &quot;\f201&quot;; }
.ion-magnet:before { content: &quot;\f2a0&quot;; }
.ion-male:before { content: &quot;\f2a1&quot;; }
.ion-man:before { content: &quot;\f202&quot;; }
.ion-map:before { content: &quot;\f203&quot;; }
.ion-medkit:before { content: &quot;\f2a2&quot;; }
.ion-merge:before { content: &quot;\f33f&quot;; }
.ion-mic-a:before { content: &quot;\f204&quot;; }
.ion-mic-b:before { content: &quot;\f205&quot;; }
.ion-mic-c:before { content: &quot;\f206&quot;; }
.ion-minus:before { content: &quot;\f209&quot;; }
.ion-minus-circled:before { content: &quot;\f207&quot;; }
.ion-minus-round:before { content: &quot;\f208&quot;; }
.ion-model-s:before { content: &quot;\f2c1&quot;; }
.ion-monitor:before { content: &quot;\f20a&quot;; }
.ion-more:before { content: &quot;\f20b&quot;; }
.ion-mouse:before { content: &quot;\f340&quot;; }
.ion-music-note:before { content: &quot;\f20c&quot;; }
.ion-navicon:before { content: &quot;\f20e&quot;; }
.ion-navicon-round:before { content: &quot;\f20d&quot;; }
.ion-navigate:before { content: &quot;\f2a3&quot;; }
.ion-network:before { content: &quot;\f341&quot;; }
.ion-no-smoking:before { content: &quot;\f2c2&quot;; }
.ion-nuclear:before { content: &quot;\f2a4&quot;; }
.ion-outlet:before { content: &quot;\f342&quot;; }
.ion-paintbrush:before { content: &quot;\f4d5&quot;; }
.ion-paintbucket:before { content: &quot;\f4d6&quot;; }
.ion-paper-airplane:before { content: &quot;\f2c3&quot;; }
.ion-paperclip:before { content: &quot;\f20f&quot;; }
.ion-pause:before { content: &quot;\f210&quot;; }
.ion-person:before { content: &quot;\f213&quot;; }
.ion-person-add:before { content: &quot;\f211&quot;; }
.ion-person-stalker:before { content: &quot;\f212&quot;; }
.ion-pie-graph:before { content: &quot;\f2a5&quot;; }
.ion-pin:before { content: &quot;\f2a6&quot;; }
.ion-pinpoint:before { content: &quot;\f2a7&quot;; }
.ion-pizza:before { content: &quot;\f2a8&quot;; }
.ion-plane:before { content: &quot;\f214&quot;; }
.ion-planet:before { content: &quot;\f343&quot;; }
.ion-play:before { content: &quot;\f215&quot;; }
.ion-playstation:before { content: &quot;\f30a&quot;; }
.ion-plus:before { content: &quot;\f218&quot;; }
.ion-plus-circled:before { content: &quot;\f216&quot;; }
.ion-plus-round:before { content: &quot;\f217&quot;; }
.ion-podium:before { content: &quot;\f344&quot;; }
.ion-pound:before { content: &quot;\f219&quot;; }
.ion-power:before { content: &quot;\f2a9&quot;; }
.ion-pricetag:before { content: &quot;\f2aa&quot;; }
.ion-pricetags:before { content: &quot;\f2ab&quot;; }
.ion-printer:before { content: &quot;\f21a&quot;; }
.ion-pull-request:before { content: &quot;\f345&quot;; }
.ion-qr-scanner:before { content: &quot;\f346&quot;; }
.ion-quote:before { content: &quot;\f347&quot;; }
.ion-radio-waves:before { content: &quot;\f2ac&quot;; }
.ion-record:before { content: &quot;\f21b&quot;; }
.ion-refresh:before { content: &quot;\f21c&quot;; }
.ion-reply:before { content: &quot;\f21e&quot;; }
.ion-reply-all:before { content: &quot;\f21d&quot;; }
.ion-ribbon-a:before { content: &quot;\f348&quot;; }
.ion-ribbon-b:before { content: &quot;\f349&quot;; }
.ion-sad:before { content: &quot;\f34a&quot;; }
.ion-sad-outline:before { content: &quot;\f4d7&quot;; }
.ion-scissors:before { content: &quot;\f34b&quot;; }
.ion-search:before { content: &quot;\f21f&quot;; }
.ion-settings:before { content: &quot;\f2ad&quot;; }
.ion-share:before { content: &quot;\f220&quot;; }
.ion-shuffle:before { content: &quot;\f221&quot;; }
.ion-skip-backward:before { content: &quot;\f222&quot;; }
.ion-skip-forward:before { content: &quot;\f223&quot;; }
.ion-social-android:before { content: &quot;\f225&quot;; }
.ion-social-android-outline:before { content: &quot;\f224&quot;; }
.ion-social-angular:before { content: &quot;\f4d9&quot;; }
.ion-social-angular-outline:before { content: &quot;\f4d8&quot;; }
.ion-social-apple:before { content: &quot;\f227&quot;; }
.ion-social-apple-outline:before { content: &quot;\f226&quot;; }
.ion-social-bitcoin:before { content: &quot;\f2af&quot;; }
.ion-social-bitcoin-outline:before { content: &quot;\f2ae&quot;; }
.ion-social-buffer:before { content: &quot;\f229&quot;; }
.ion-social-buffer-outline:before { content: &quot;\f228&quot;; }
.ion-social-chrome:before { content: &quot;\f4db&quot;; }
.ion-social-chrome-outline:before { content: &quot;\f4da&quot;; }
.ion-social-codepen:before { content: &quot;\f4dd&quot;; }
.ion-social-codepen-outline:before { content: &quot;\f4dc&quot;; }
.ion-social-css3:before { content: &quot;\f4df&quot;; }
.ion-social-css3-outline:before { content: &quot;\f4de&quot;; }
.ion-social-designernews:before { content: &quot;\f22b&quot;; }
.ion-social-designernews-outline:before { content: &quot;\f22a&quot;; }
.ion-social-dribbble:before { content: &quot;\f22d&quot;; }
.ion-social-dribbble-outline:before { content: &quot;\f22c&quot;; }
.ion-social-dropbox:before { content: &quot;\f22f&quot;; }
.ion-social-dropbox-outline:before { content: &quot;\f22e&quot;; }
.ion-social-euro:before { content: &quot;\f4e1&quot;; }
.ion-social-euro-outline:before { content: &quot;\f4e0&quot;; }
.ion-social-facebook:before { content: &quot;\f231&quot;; }
.ion-social-facebook-outline:before { content: &quot;\f230&quot;; }
.ion-social-foursquare:before { content: &quot;\f34d&quot;; }
.ion-social-foursquare-outline:before { content: &quot;\f34c&quot;; }
.ion-social-freebsd-devil:before { content: &quot;\f2c4&quot;; }
.ion-social-github:before { content: &quot;\f233&quot;; }
.ion-social-github-outline:before { content: &quot;\f232&quot;; }
.ion-social-google:before { content: &quot;\f34f&quot;; }
.ion-social-google-outline:before { content: &quot;\f34e&quot;; }
.ion-social-googleplus:before { content: &quot;\f235&quot;; }
.ion-social-googleplus-outline:before { content: &quot;\f234&quot;; }
.ion-social-hackernews:before { content: &quot;\f237&quot;; }
.ion-social-hackernews-outline:before { content: &quot;\f236&quot;; }
.ion-social-html5:before { content: &quot;\f4e3&quot;; }
.ion-social-html5-outline:before { content: &quot;\f4e2&quot;; }
.ion-social-instagram:before { content: &quot;\f351&quot;; }
.ion-social-instagram-outline:before { content: &quot;\f350&quot;; }
.ion-social-javascript:before { content: &quot;\f4e5&quot;; }
.ion-social-javascript-outline:before { content: &quot;\f4e4&quot;; }
.ion-social-linkedin:before { content: &quot;\f239&quot;; }
.ion-social-linkedin-outline:before { content: &quot;\f238&quot;; }
.ion-social-markdown:before { content: &quot;\f4e6&quot;; }
.ion-social-nodejs:before { content: &quot;\f4e7&quot;; }
.ion-social-octocat:before { content: &quot;\f4e8&quot;; }
.ion-social-pinterest:before { content: &quot;\f2b1&quot;; }
.ion-social-pinterest-outline:before { content: &quot;\f2b0&quot;; }
.ion-social-python:before { content: &quot;\f4e9&quot;; }
.ion-social-reddit:before { content: &quot;\f23b&quot;; }
.ion-social-reddit-outline:before { content: &quot;\f23a&quot;; }
.ion-social-rss:before { content: &quot;\f23d&quot;; }
.ion-social-rss-outline:before { content: &quot;\f23c&quot;; }
.ion-social-sass:before { content: &quot;\f4ea&quot;; }
.ion-social-skype:before { content: &quot;\f23f&quot;; }
.ion-social-skype-outline:before { content: &quot;\f23e&quot;; }
.ion-social-snapchat:before { content: &quot;\f4ec&quot;; }
.ion-social-snapchat-outline:before { content: &quot;\f4eb&quot;; }
.ion-social-tumblr:before { content: &quot;\f241&quot;; }
.ion-social-tumblr-outline:before { content: &quot;\f240&quot;; }
.ion-social-tux:before { content: &quot;\f2c5&quot;; }
.ion-social-twitch:before { content: &quot;\f4ee&quot;; }
.ion-social-twitch-outline:before { content: &quot;\f4ed&quot;; }
.ion-social-twitter:before { content: &quot;\f243&quot;; }
.ion-social-twitter-outline:before { content: &quot;\f242&quot;; }
.ion-social-usd:before { content: &quot;\f353&quot;; }
.ion-social-usd-outline:before { content: &quot;\f352&quot;; }
.ion-social-vimeo:before { content: &quot;\f245&quot;; }
.ion-social-vimeo-outline:before { content: &quot;\f244&quot;; }
.ion-social-whatsapp:before { content: &quot;\f4f0&quot;; }
.ion-social-whatsapp-outline:before { content: &quot;\f4ef&quot;; }
.ion-social-windows:before { content: &quot;\f247&quot;; }
.ion-social-windows-outline:before { content: &quot;\f246&quot;; }
.ion-social-wordpress:before { content: &quot;\f249&quot;; }
.ion-social-wordpress-outline:before { content: &quot;\f248&quot;; }
.ion-social-yahoo:before { content: &quot;\f24b&quot;; }
.ion-social-yahoo-outline:before { content: &quot;\f24a&quot;; }
.ion-social-yen:before { content: &quot;\f4f2&quot;; }
.ion-social-yen-outline:before { content: &quot;\f4f1&quot;; }
.ion-social-youtube:before { content: &quot;\f24d&quot;; }
.ion-social-youtube-outline:before { content: &quot;\f24c&quot;; }
.ion-soup-can:before { content: &quot;\f4f4&quot;; }
.ion-soup-can-outline:before { content: &quot;\f4f3&quot;; }
.ion-speakerphone:before { content: &quot;\f2b2&quot;; }
.ion-speedometer:before { content: &quot;\f2b3&quot;; }
.ion-spoon:before { content: &quot;\f2b4&quot;; }
.ion-star:before { content: &quot;\f24e&quot;; }
.ion-stats-bars:before { content: &quot;\f2b5&quot;; }
.ion-steam:before { content: &quot;\f30b&quot;; }
.ion-stop:before { content: &quot;\f24f&quot;; }
.ion-thermometer:before { content: &quot;\f2b6&quot;; }
.ion-thumbsdown:before { content: &quot;\f250&quot;; }
.ion-thumbsup:before { content: &quot;\f251&quot;; }
.ion-toggle:before { content: &quot;\f355&quot;; }
.ion-toggle-filled:before { content: &quot;\f354&quot;; }
.ion-transgender:before { content: &quot;\f4f5&quot;; }
.ion-trash-a:before { content: &quot;\f252&quot;; }
.ion-trash-b:before { content: &quot;\f253&quot;; }
.ion-trophy:before { content: &quot;\f356&quot;; }
.ion-tshirt:before { content: &quot;\f4f7&quot;; }
.ion-tshirt-outline:before { content: &quot;\f4f6&quot;; }
.ion-umbrella:before { content: &quot;\f2b7&quot;; }
.ion-university:before { content: &quot;\f357&quot;; }
.ion-unlocked:before { content: &quot;\f254&quot;; }
.ion-upload:before { content: &quot;\f255&quot;; }
.ion-usb:before { content: &quot;\f2b8&quot;; }
.ion-videocamera:before { content: &quot;\f256&quot;; }
.ion-volume-high:before { content: &quot;\f257&quot;; }
.ion-volume-low:before { content: &quot;\f258&quot;; }
.ion-volume-medium:before { content: &quot;\f259&quot;; }
.ion-volume-mute:before { content: &quot;\f25a&quot;; }
.ion-wand:before { content: &quot;\f358&quot;; }
.ion-waterdrop:before { content: &quot;\f25b&quot;; }
.ion-wifi:before { content: &quot;\f25c&quot;; }
.ion-wineglass:before { content: &quot;\f2b9&quot;; }
.ion-woman:before { content: &quot;\f25d&quot;; }
.ion-wrench:before { content: &quot;\f2ba&quot;; }
.ion-xbox:before { content: &quot;\f30c&quot;; }
/* from dash board */
.body-color
{
    background-color: #F3F3EB;
    height: 100%;

}
.left-bar
{
    background-color: #47250E;
    -webkit-box-shadow: 0 0 12px 0 rgba(113,113,113,0.5), 1px -25px 10px 0 #D9E4EC;
            box-shadow: 0 0 12px 0 rgba(113,113,113,0.5), 1px -25px 10px 0 #D9E4EC;
    position: fixed;
    left: 0;
    bottom: 0;
    height: 100%;
    width: 223px;
    padding: 241px 0px 0px 43px;
    /* flex-direction: row; */
}
.left-bar-detail
{
    overflow: hidden;
    width:100%;
    height: 300px;
}
.left-bar-bottom{
      position: absolute;
  bottom: 16px;
  color: rgba(255,255,255,0.87);
}
.left-bar-head
{
    position: absolute;
    z-index: 100;
    left: 0;
    top: 0;
    height: 200px;	
    width: 223px;
    background-color: #25150A;	
    color: rgba(255,255,255,0.87);
    font-size: 16px;	
    font-weight: 500;	
    line-height: 24px;
}
.left-bar-head-inside
{
    padding-top: 26px;
    color: rgba(255,255,255,0.87);
    text-align:center;
}
.inline
{
    float: left;
}
.left-bar-menu
{
    position: relative;
    color: rgba(255,255,255,0.87);
    padding-bottom: 27px;
}
.unread
{
    font-size: 15px;
    background-color:#E22B2B;
    color: #fff;
    width: 23px;
    height: 23px;
    border-radius: 50%;
    text-align: center;
    position: absolute;
    top: 0;
    right: 55px;
}
.active-menu
{
    color: #abd10f ;
}
.input-search
{
    height: 83px;	
    width: 100%;	
    background-color: #FFFFFF;	
}
.right-side
{
    margin-left: 223px;
    height: 100%;
}
.search-input
{
    height: 81px;
}
.search-icon
{
    font-size: 30px;
    position: absolute;
    top: 20px;
    right: 50px;
}
.dashboard-card
{
    /* min-height: 898px;	 */
    /* height: calc(100vh - 4rem); */
    /* height: 100%; */
    position: relative;
   /*  width: 100%;*/	 
    background-color: #fff;	
    -webkit-box-shadow: -3px 2px 2px 0 rgba(119,151,178,0.16);	
            box-shadow: -3px 2px 2px 0 rgba(119,151,178,0.16);
    margin: 20px;
    padding: 18px 32px 18px 32px;
}
.dashboard-card1
{
    /* min-height: 898px;    */
    /* height: calc(100vh - 4rem); */
    /* height: 100%; */
/*    border: 1px solid #a2a2a2;
    border-radius: 10px;*/
    position: relative;
     width: 100%;     
    background-color: #fff; 
    
    -webkit-box-shadow: -3px 2px 2px 0 rgba(119,151,178,0.16); 
    
            box-shadow: -3px 2px 2px 0 rgba(119,151,178,0.16);
    margin: 20px;
    padding: 18px 32px 18px 32px;
}
.text-center
{
    text-align: center !important;
    vertical-align: middle !important;
    margin-top: 36px;
}
.text-center1
{
    text-align: center !important;
    vertical-align: middle !important;
     margin-top: 40px;
}
.cancel-order-text
{
    color:red;
}
.confirm-order-text
{
    color:green;
}
.page-item.active .page-link 
{
    z-index: 2;
    color: #fff;
    background-color: #22923E;
    border-color: #22923E;
}
.page-link 
{
    position: relative;
    display: block;
    padding: 0.5rem 0.75rem;
    margin-left: -1px;
    line-height: 1.25;
    color: #22923E;
    background-color: #fff;
    border: 1px solid #ddd;
}
/* Fixed sponsive for deign */
.main
{
    position: fixed;
    z-index: 100;
    /* left: 0;
    top: 0; */
    height: 100%;
    width: 223px;
    background-color: #25150A;
    color: rgba(255,255,255,0.87);
    font-size: 16px;
    font-weight: 500;
    line-height: 24px;
}
.main .icon {
        display: none;
      }
.topnav {
        overflow: hidden;
        background-color: #47250E;
        display: none;
      }
.topnav a {
        float: left;
        display: block;
        color: #f2f2f2;
        text-align: center;
        padding: 14px 16px;
        text-decoration: none;
        font-size: 17px;
      }
.topnav a:hover {
        background-color: #ddd;
        color: black;
      }
/* .active {
        background-color: #25150A;	
        color: white;
      } */
.topnav .icon {
        display: none;
      }
@media screen and (max-width: 900px)
{   
    /* .search-input
    {
        height: 46px;
        padding-right: 100px;
    }
    .left-bar:not(:first-child) {display: none;}
    
    .main a.icon {
        display: block;
    vertical-align: top;
    text-align: center !important;
    vertical-align: middle !important;
    padding-top: 10px;
      }
      .left-bar-head
    {
        display: none;
    }
    .search-icon
    {
        font-size: 30px;
        position: absolute;
        top: 2px;
        right: 120px;
    }
    .main{
        
        position: absolute;
        height: 46px;
        right:0;
        width: 100px;
        background-color: #abd10f
    } */
    .right-side
    {
        margin-left:0px;
    }
}
@media screen and (max-width: 900px) {
    .topnav a {display: none;}
   
    .topnav a.icon {
      float: right;
      display: block;
    }
    .topnav {
        display: block;
    }
    .main{
        display: none;
    }
  }
.search-icon
  {
      font-size: 30px;
      position: absolute;
      top: 50px;
      right: 30px;
  }
.search-input
    {
        height: 46px;
        padding-right: 100px;
    }
@media screen and (max-width: 900px) {
    .topnav.responsive {position: relative;}
    .topnav.responsive .icon {
      position: absolute;
      right: 0;
      top: 0;
    }
    .topnav.responsive a {
      float: none;
      display: block;
      text-align: left;
    }
  }
.add-farmer-button
  {
      margin-right: 20px;
      background-color: #697B8C;
      color: #FFFFFF;
      font-size: 20px;
      width: 184px;
      border-bottom: solid 2px #374655;
  }
.farmer-all-card
  {
      max-width: 1100px;
  }
.farmer-card
  {
      height: 400px;	
      width: 300px;	
      background-color: #FFFFFF;	
      -webkit-box-shadow: -3px 2px 2px 0 rgba(196,196,196,0.16);	
              box-shadow: -3px 2px 2px 0 rgba(196,196,196,0.16);
      float: left;
      margin: 20px 10px 4px 10px;
  }
.farmer-image img
  {
      width: 300px;
      height:  200px;
      -o-object-fit: cover;
         object-fit: cover;
      
  }
.farmer-detail
  {
      width: 300px;
      color: #697B8C;	
      padding: 8px;	
      line-height: 30px;
  }
.farmer-name
  {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
  }
.location-icon
  {
      font-size: 20px;
      padding-right: 10px;
      float: left;
  }
.province
  {
      font-size:14px;
  }
.add-farmer-input
  {
      width: 390px;
  }
#farmerPicture
  {
      -webkit-box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.05);
              box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.05);
      width:155px;
      height: 155px;
      position: relative;
      margin:10px;
  }
.close
  {
      position: absolute;
      right: 5px;
      text-decoration: none;
      text-shadow: 0 1px 0 #fff;
      top: 5px;
  }
.close:hover
  {
      cursor: pointer;
  }
.thumb-post img {
    -o-object-fit: cover;
       object-fit: cover;
  }/*!
 * Bootstrap v4.0.0-beta.2 (https://getbootstrap.com)
 * Copyright 2011-2017 The Bootstrap Authors
 * Copyright 2011-2017 Twitter, Inc.
 * Licensed under MIT (https://github.com/twbs/bootstrap/blob/master/LICENSE)
 */
:root {
  --blue: #007bff;
  --indigo: #6610f2;
  --purple: #6f42c1;
  --pink: #e83e8c;
  --red: #dc3545;
  --orange: #fd7e14;
  --yellow: #ffc107;
  --green: #28a745;
  --teal: #20c997;
  --cyan: #17a2b8;
  --white: #fff;
  --gray: #868e96;
  --gray-dark: #343a40;
  --primary: #007bff;
  --secondary: #868e96;
  --success: #28a745;
  --info: #17a2b8;
  --warning: #ffc107;
  --danger: #dc3545;
  --light: #f8f9fa;
  --dark: #343a40;
  --breakpoint-xs: 0;
  --breakpoint-sm: 576px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 992px;
  --breakpoint-xl: 1200px;
  --font-family-sans-serif: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Helvetica Neue&quot;, Arial, sans-serif, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Symbol&quot;;
  --font-family-monospace: &quot;SFMono-Regular&quot;, Menlo, Monaco, Consolas, &quot;Liberation Mono&quot;, &quot;Courier New&quot;, monospace;
}
@media print {
  *,
  *::before,
  *::after {
    text-shadow: none !important;
    box-shadow: none !important;
  }
  a,
  a:visited {
    text-decoration: underline;
  }
  abbr[title]::after {
    content: &quot; (&quot; attr(title) &quot;)&quot;;
  }
  pre {
    white-space: pre-wrap !important;
  }
  pre,
  blockquote {
    border: 1px solid #999;
    page-break-inside: avoid;
  }
  thead {
    display: table-header-group;
  }
  tr,
  img {
    page-break-inside: avoid;
  }
  p,
  h2,
  h3 {
    orphans: 3;
    widows: 3;
  }
  h2,
  h3 {
    page-break-after: avoid;
  }
  .navbar {
    display: none;
  }
  .badge {
    border: 1px solid #000;
  }
  .table {
    border-collapse: collapse !important;
  }
  .table td,
  .table th {
    background-color: #fff !important;
  }
  .table-bordered th,
  .table-bordered td {
    border: 1px solid #ddd !important;
  }
}
*,
*::before,
*::after {
  box-sizing: border-box;
}
html {
  font-family: sans-serif;
  line-height: 1.15;
  -webkit-text-size-adjust: 100%;
  -ms-text-size-adjust: 100%;
  -ms-overflow-style: scrollbar;
  -webkit-tap-highlight-color: transparent;
}
@-ms-viewport {
  width: device-width;
}
article, aside, dialog, figcaption, figure, footer, header, hgroup, main, nav, section {
  display: block;
}
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Helvetica Neue&quot;, Arial, sans-serif, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Symbol&quot;;
  font-size: 1rem;
  font-weight: 400;
  line-height: 1.5;
  color: #212529;
  text-align: left;
  background-color: #fff;
}
[tabindex=&quot;-1&quot;]:focus {
  outline: none !important;
}
hr {
  box-sizing: content-box;
  height: 0;
  overflow: visible;
}
h1, h2, h3, h4, h5, h6 {
  margin-top: 0;
  margin-bottom: 0.5rem;
}
p {
  margin-top: 0;
  margin-bottom: 1rem;
}
abbr[title],
abbr[data-original-title] {
  text-decoration: underline;
  -webkit-text-decoration: underline dotted;
          text-decoration: underline dotted;
  cursor: help;
  border-bottom: 0;
}
address {
  margin-bottom: 1rem;
  font-style: normal;
  line-height: inherit;
}
ol,
ul,
dl {
  margin-top: 0;
  margin-bottom: 1rem;
}
ol ol,
ul ul,
ol ul,
ul ol {
  margin-bottom: 0;
}
dt {
  font-weight: 700;
}
dd {
  margin-bottom: .5rem;
  margin-left: 0;
}
blockquote {
  margin: 0 0 1rem;
}
dfn {
  font-style: italic;
}
b,
strong {
  font-weight: bolder;
}
small {
  font-size: 80%;
}
sub,
sup {
  position: relative;
  font-size: 75%;
  line-height: 0;
  vertical-align: baseline;
}
sub {
  bottom: -.25em;
}
sup {
  top: -.5em;
}
a {
  color: #007bff;
  text-decoration: none;
  background-color: transparent;
  -webkit-text-decoration-skip: objects;
}
a:hover {
  color: #0056b3;
  text-decoration: underline;
}
a:not([href]):not([tabindex]) {
  color: inherit;
  text-decoration: none;
}
a:not([href]):not([tabindex]):focus, a:not([href]):not([tabindex]):hover {
  color: inherit;
  text-decoration: none;
}
a:not([href]):not([tabindex]):focus {
  outline: 0;
}
pre,
code,
kbd,
samp {
  font-family: monospace, monospace;
  font-size: 1em;
}
pre {
  margin-top: 0;
  margin-bottom: 1rem;
  overflow: auto;
  -ms-overflow-style: scrollbar;
}
figure {
  margin: 0 0 1rem;
}
img {
  vertical-align: middle;
  border-style: none;
}
svg:not(:root) {
  overflow: hidden;
}
a,
area,
button,
[role=&quot;button&quot;],
input:not([type=&quot;range&quot;]),
label,
select,
summary,
textarea {
  -ms-touch-action: manipulation;
      touch-action: manipulation;
}
table {
  border-collapse: collapse;
}
caption {
  padding-top: 0.75rem;
  padding-bottom: 0.75rem;
  color: #868e96;
  text-align: left;
  caption-side: bottom;
}
th {
  text-align: inherit;
}
label {
  display: inline-block;
  margin-bottom: .5rem;
}
button {
  border-radius: 0;
}
button:focus {
  outline: 1px dotted;
  outline: 5px auto -webkit-focus-ring-color;
}
input,
button,
select,
optgroup,
textarea {
  margin: 0;
  font-family: inherit;
  font-size: inherit;
  line-height: inherit;
}
button,
input {
  overflow: visible;
}
button,
select {
  text-transform: none;
}
button,
html [type=&quot;button&quot;],
[type=&quot;reset&quot;],
[type=&quot;submit&quot;] {
  -webkit-appearance: button;
}
button::-moz-focus-inner,
[type=&quot;button&quot;]::-moz-focus-inner,
[type=&quot;reset&quot;]::-moz-focus-inner,
[type=&quot;submit&quot;]::-moz-focus-inner {
  padding: 0;
  border-style: none;
}
input[type=&quot;radio&quot;],
input[type=&quot;checkbox&quot;] {
  box-sizing: border-box;
  padding: 0;
}
input[type=&quot;date&quot;],
input[type=&quot;time&quot;],
input[type=&quot;datetime-local&quot;],
input[type=&quot;month&quot;] {
  -webkit-appearance: listbox;
}
textarea {
  overflow: auto;
  resize: vertical;
}
fieldset {
  min-width: 0;
  padding: 0;
  margin: 0;
  border: 0;
}
legend {
  display: block;
  width: 100%;
  max-width: 100%;
  padding: 0;
  margin-bottom: .5rem;
  font-size: 1.5rem;
  line-height: inherit;
  color: inherit;
  white-space: normal;
}
progress {
  vertical-align: baseline;
}
[type=&quot;number&quot;]::-webkit-inner-spin-button,
[type=&quot;number&quot;]::-webkit-outer-spin-button {
  height: auto;
}
[type=&quot;search&quot;] {
  outline-offset: -2px;
  -webkit-appearance: none;
}
[type=&quot;search&quot;]::-webkit-search-cancel-button,
[type=&quot;search&quot;]::-webkit-search-decoration {
  -webkit-appearance: none;
}
::-webkit-file-upload-button {
  font: inherit;
  -webkit-appearance: button;
}
output {
  display: inline-block;
}
summary {
  display: list-item;
}
template {
  display: none;
}
[hidden] {
  display: none !important;
}
h1, h2, h3, h4, h5, h6,
.h1, .h2, .h3, .h4, .h5, .h6 {
  margin-bottom: 0.5rem;
  font-family: inherit;
  font-weight: 500;
  line-height: 1.2;
  color: inherit;
}
h1, .h1 {
  font-size: 2.5rem;
}
h2, .h2 {
  font-size: 2rem;
}
h3, .h3 {
  font-size: 1.75rem;
}
h4, .h4 {
  font-size: 1.5rem;
}
h5, .h5 {
  font-size: 1.25rem;
}
h6, .h6 {
  font-size: 1rem;
}
.lead {
  font-size: 1.25rem;
  font-weight: 300;
}
.display-1 {
  font-size: 6rem;
  font-weight: 300;
  line-height: 1.2;
}
.display-2 {
  font-size: 5.5rem;
  font-weight: 300;
  line-height: 1.2;
}
.display-3 {
  font-size: 4.5rem;
  font-weight: 300;
  line-height: 1.2;
}
.display-4 {
  font-size: 3.5rem;
  font-weight: 300;
  line-height: 1.2;
}
hr {
  margin-top: 1rem;
  margin-bottom: 1rem;
  border: 0;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}
small,
.small {
  font-size: 80%;
  font-weight: 400;
}
mark,
.mark {
  padding: 0.2em;
  background-color: #fcf8e3;
}
.list-unstyled {
  padding-left: 0;
  list-style: none;
}
.list-inline {
  padding-left: 0;
  list-style: none;
}
.list-inline-item {
  display: inline-block;
}
.list-inline-item:not(:last-child) {
  margin-right: 5px;
}
.initialism {
  font-size: 90%;
  text-transform: uppercase;
}
.blockquote {
  margin-bottom: 1rem;
  font-size: 1.25rem;
}
.blockquote-footer {
  display: block;
  font-size: 80%;
  color: #868e96;
}
.blockquote-footer::before {
  content: &quot;\2014 \00A0&quot;;
}
.img-fluid {
  max-width: 100%;
  height: auto;
}
.img-thumbnail {
  padding: 0.25rem;
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 0.25rem;
  transition: all 0.2s ease-in-out;
  max-width: 100%;
  height: auto;
}
.figure {
  display: inline-block;
}
.figure-img {
  margin-bottom: 0.5rem;
  line-height: 1;
}
.figure-caption {
  font-size: 90%;
  color: #868e96;
}
code,
kbd,
pre,
samp {
  font-family: &quot;SFMono-Regular&quot;, Menlo, Monaco, Consolas, &quot;Liberation Mono&quot;, &quot;Courier New&quot;, monospace;
}
code {
  padding: 0.2rem 0.4rem;
  font-size: 90%;
  color: #bd4147;
  background-color: #f8f9fa;
  border-radius: 0.25rem;
}
a > code {
  padding: 0;
  color: inherit;
  background-color: inherit;
}
kbd {
  padding: 0.2rem 0.4rem;
  font-size: 90%;
  color: #fff;
  background-color: #212529;
  border-radius: 0.2rem;
}
kbd kbd {
  padding: 0;
  font-size: 100%;
  font-weight: 700;
}
pre {
  display: block;
  margin-top: 0;
  margin-bottom: 1rem;
  font-size: 90%;
  color: #212529;
}
pre code {
  padding: 0;
  font-size: inherit;
  color: inherit;
  background-color: transparent;
  border-radius: 0;
}
.pre-scrollable {
  max-height: 340px;
  overflow-y: scroll;
}
.container {
  width: 100%;
  padding-right: 15px;
  padding-left: 15px;
  margin-right: auto;
  margin-left: auto;
}
@media (min-width: 576px) {
  .container {
    max-width: 540px;
  }
}
@media (min-width: 768px) {
  .container {
    max-width: 720px;
  }
}
@media (min-width: 992px) {
  .container {
    max-width: 960px;
  }
}
@media (min-width: 1200px) {
  .container {
    max-width: 1140px;
  }
}
.container-fluid {
  width: 100%;
  padding-right: 15px;
  padding-left: 15px;
  margin-right: auto;
  margin-left: auto;
}
.row {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  margin-right: -15px;
  margin-left: -15px;
}
.no-gutters {
  margin-right: 0;
  margin-left: 0;
}
.no-gutters > .col,
.no-gutters > [class*=&quot;col-&quot;] {
  padding-right: 0;
  padding-left: 0;
}
.col-1, .col-2, .col-3, .col-4, .col-5, .col-6, .col-7, .col-8, .col-9, .col-10, .col-11, .col-12, .col,
.col-auto, .col-sm-1, .col-sm-2, .col-sm-3, .col-sm-4, .col-sm-5, .col-sm-6, .col-sm-7, .col-sm-8, .col-sm-9, .col-sm-10, .col-sm-11, .col-sm-12, .col-sm,
.col-sm-auto, .col-md-1, .col-md-2, .col-md-3, .col-md-4, .col-md-5, .col-md-6, .col-md-7, .col-md-8, .col-md-9, .col-md-10, .col-md-11, .col-md-12, .col-md,
.col-md-auto, .col-lg-1, .col-lg-2, .col-lg-3, .col-lg-4, .col-lg-5, .col-lg-6, .col-lg-7, .col-lg-8, .col-lg-9, .col-lg-10, .col-lg-11, .col-lg-12, .col-lg,
.col-lg-auto, .col-xl-1, .col-xl-2, .col-xl-3, .col-xl-4, .col-xl-5, .col-xl-6, .col-xl-7, .col-xl-8, .col-xl-9, .col-xl-10, .col-xl-11, .col-xl-12, .col-xl,
.col-xl-auto {
  position: relative;
  width: 100%;
  min-height: 1px;
  padding-right: 15px;
  padding-left: 15px;
}
.col {
  -ms-flex-preferred-size: 0;
      flex-basis: 0;
  -ms-flex-positive: 1;
      flex-grow: 1;
  max-width: 100%;
}
.col-auto {
  -ms-flex: 0 0 auto;
      flex: 0 0 auto;
  width: auto;
  max-width: none;
}
.col-1 {
  -ms-flex: 0 0 8.333333%;
      flex: 0 0 8.333333%;
  max-width: 8.333333%;
}
.col-2 {
  -ms-flex: 0 0 16.666667%;
      flex: 0 0 16.666667%;
  max-width: 16.666667%;
}
.col-3 {
  -ms-flex: 0 0 25%;
      flex: 0 0 25%;
  max-width: 25%;
}
.col-4 {
  -ms-flex: 0 0 33.333333%;
      flex: 0 0 33.333333%;
  max-width: 33.333333%;
}
.col-5 {
  -ms-flex: 0 0 41.666667%;
      flex: 0 0 41.666667%;
  max-width: 41.666667%;
}
.col-6 {
  -ms-flex: 0 0 50%;
      flex: 0 0 50%;
  max-width: 50%;
}
.col-7 {
  -ms-flex: 0 0 58.333333%;
      flex: 0 0 58.333333%;
  max-width: 58.333333%;
}
.col-8 {
  -ms-flex: 0 0 66.666667%;
      flex: 0 0 66.666667%;
  max-width: 66.666667%;
}
.col-9 {
  -ms-flex: 0 0 75%;
      flex: 0 0 75%;
  max-width: 75%;
}
.col-10 {
  -ms-flex: 0 0 83.333333%;
      flex: 0 0 83.333333%;
  max-width: 83.333333%;
}
.col-11 {
  -ms-flex: 0 0 91.666667%;
      flex: 0 0 91.666667%;
  max-width: 91.666667%;
}
.col-12 {
  -ms-flex: 0 0 100%;
      flex: 0 0 100%;
  max-width: 100%;
}
.order-first {
  -ms-flex-order: -1;
      order: -1;
}
.order-1 {
  -ms-flex-order: 1;
      order: 1;
}
.order-2 {
  -ms-flex-order: 2;
      order: 2;
}
.order-3 {
  -ms-flex-order: 3;
      order: 3;
}
.order-4 {
  -ms-flex-order: 4;
      order: 4;
}
.order-5 {
  -ms-flex-order: 5;
      order: 5;
}
.order-6 {
  -ms-flex-order: 6;
      order: 6;
}
.order-7 {
  -ms-flex-order: 7;
      order: 7;
}
.order-8 {
  -ms-flex-order: 8;
      order: 8;
}
.order-9 {
  -ms-flex-order: 9;
      order: 9;
}
.order-10 {
  -ms-flex-order: 10;
      order: 10;
}
.order-11 {
  -ms-flex-order: 11;
      order: 11;
}
.order-12 {
  -ms-flex-order: 12;
      order: 12;
}
.offset-1 {
  margin-left: 8.333333%;
}
.offset-2 {
  margin-left: 16.666667%;
}
.offset-3 {
  margin-left: 25%;
}
.offset-4 {
  margin-left: 33.333333%;
}
.offset-5 {
  margin-left: 41.666667%;
}
.offset-6 {
  margin-left: 50%;
}
.offset-7 {
  margin-left: 58.333333%;
}
.offset-8 {
  margin-left: 66.666667%;
}
.offset-9 {
  margin-left: 75%;
}
.offset-10 {
  margin-left: 83.333333%;
}
.offset-11 {
  margin-left: 91.666667%;
}
@media (min-width: 576px) {
  .col-sm {
    -ms-flex-preferred-size: 0;
        flex-basis: 0;
    -ms-flex-positive: 1;
        flex-grow: 1;
    max-width: 100%;
  }
  .col-sm-auto {
    -ms-flex: 0 0 auto;
        flex: 0 0 auto;
    width: auto;
    max-width: none;
  }
  .col-sm-1 {
    -ms-flex: 0 0 8.333333%;
        flex: 0 0 8.333333%;
    max-width: 8.333333%;
  }
  .col-sm-2 {
    -ms-flex: 0 0 16.666667%;
        flex: 0 0 16.666667%;
    max-width: 16.666667%;
  }
  .col-sm-3 {
    -ms-flex: 0 0 25%;
        flex: 0 0 25%;
    max-width: 25%;
  }
  .col-sm-4 {
    -ms-flex: 0 0 33.333333%;
        flex: 0 0 33.333333%;
    max-width: 33.333333%;
  }
  .col-sm-5 {
    -ms-flex: 0 0 41.666667%;
        flex: 0 0 41.666667%;
    max-width: 41.666667%;
  }
  .col-sm-6 {
    -ms-flex: 0 0 50%;
        flex: 0 0 50%;
    max-width: 50%;
  }
  .col-sm-7 {
    -ms-flex: 0 0 58.333333%;
        flex: 0 0 58.333333%;
    max-width: 58.333333%;
  }
  .col-sm-8 {
    -ms-flex: 0 0 66.666667%;
        flex: 0 0 66.666667%;
    max-width: 66.666667%;
  }
  .col-sm-9 {
    -ms-flex: 0 0 75%;
        flex: 0 0 75%;
    max-width: 75%;
  }
  .col-sm-10 {
    -ms-flex: 0 0 83.333333%;
        flex: 0 0 83.333333%;
    max-width: 83.333333%;
  }
  .col-sm-11 {
    -ms-flex: 0 0 91.666667%;
        flex: 0 0 91.666667%;
    max-width: 91.666667%;
  }
  .col-sm-12 {
    -ms-flex: 0 0 100%;
        flex: 0 0 100%;
    max-width: 100%;
  }
  .order-sm-first {
    -ms-flex-order: -1;
        order: -1;
  }
  .order-sm-1 {
    -ms-flex-order: 1;
        order: 1;
  }
  .order-sm-2 {
    -ms-flex-order: 2;
        order: 2;
  }
  .order-sm-3 {
    -ms-flex-order: 3;
        order: 3;
  }
  .order-sm-4 {
    -ms-flex-order: 4;
        order: 4;
  }
  .order-sm-5 {
    -ms-flex-order: 5;
        order: 5;
  }
  .order-sm-6 {
    -ms-flex-order: 6;
        order: 6;
  }
  .order-sm-7 {
    -ms-flex-order: 7;
        order: 7;
  }
  .order-sm-8 {
    -ms-flex-order: 8;
        order: 8;
  }
  .order-sm-9 {
    -ms-flex-order: 9;
        order: 9;
  }
  .order-sm-10 {
    -ms-flex-order: 10;
        order: 10;
  }
  .order-sm-11 {
    -ms-flex-order: 11;
        order: 11;
  }
  .order-sm-12 {
    -ms-flex-order: 12;
        order: 12;
  }
  .offset-sm-0 {
    margin-left: 0;
  }
  .offset-sm-1 {
    margin-left: 8.333333%;
  }
  .offset-sm-2 {
    margin-left: 16.666667%;
  }
  .offset-sm-3 {
    margin-left: 25%;
  }
  .offset-sm-4 {
    margin-left: 33.333333%;
  }
  .offset-sm-5 {
    margin-left: 41.666667%;
  }
  .offset-sm-6 {
    margin-left: 50%;
  }
  .offset-sm-7 {
    margin-left: 58.333333%;
  }
  .offset-sm-8 {
    margin-left: 66.666667%;
  }
  .offset-sm-9 {
    margin-left: 75%;
  }
  .offset-sm-10 {
    margin-left: 83.333333%;
  }
  .offset-sm-11 {
    margin-left: 91.666667%;
  }
}
@media (min-width: 768px) {
  .col-md {
    -ms-flex-preferred-size: 0;
        flex-basis: 0;
    -ms-flex-positive: 1;
        flex-grow: 1;
    max-width: 100%;
  }
  .col-md-auto {
    -ms-flex: 0 0 auto;
        flex: 0 0 auto;
    width: auto;
    max-width: none;
  }
  .col-md-1 {
    -ms-flex: 0 0 8.333333%;
        flex: 0 0 8.333333%;
    max-width: 8.333333%;
  }
  .col-md-2 {
    -ms-flex: 0 0 16.666667%;
        flex: 0 0 16.666667%;
    max-width: 16.666667%;
  }
  .col-md-3 {
    -ms-flex: 0 0 25%;
        flex: 0 0 25%;
    max-width: 25%;
  }
  .col-md-4 {
    -ms-flex: 0 0 33.333333%;
        flex: 0 0 33.333333%;
    max-width: 33.333333%;
  }
  .col-md-5 {
    -ms-flex: 0 0 41.666667%;
        flex: 0 0 41.666667%;
    max-width: 41.666667%;
  }
  .col-md-6 {
    -ms-flex: 0 0 50%;
        flex: 0 0 50%;
    max-width: 50%;
  }
  .col-md-7 {
    -ms-flex: 0 0 58.333333%;
        flex: 0 0 58.333333%;
    max-width: 58.333333%;
  }
  .col-md-8 {
    -ms-flex: 0 0 66.666667%;
        flex: 0 0 66.666667%;
    max-width: 66.666667%;
  }
  .col-md-9 {
    -ms-flex: 0 0 75%;
        flex: 0 0 75%;
    max-width: 75%;
  }
  .col-md-10 {
    -ms-flex: 0 0 83.333333%;
        flex: 0 0 83.333333%;
    max-width: 83.333333%;
  }
  .col-md-11 {
    -ms-flex: 0 0 91.666667%;
        flex: 0 0 91.666667%;
    max-width: 91.666667%;
  }
  .col-md-12 {
    -ms-flex: 0 0 100%;
        flex: 0 0 100%;
    max-width: 100%;
  }
  .order-md-first {
    -ms-flex-order: -1;
        order: -1;
  }
  .order-md-1 {
    -ms-flex-order: 1;
        order: 1;
  }
  .order-md-2 {
    -ms-flex-order: 2;
        order: 2;
  }
  .order-md-3 {
    -ms-flex-order: 3;
        order: 3;
  }
  .order-md-4 {
    -ms-flex-order: 4;
        order: 4;
  }
  .order-md-5 {
    -ms-flex-order: 5;
        order: 5;
  }
  .order-md-6 {
    -ms-flex-order: 6;
        order: 6;
  }
  .order-md-7 {
    -ms-flex-order: 7;
        order: 7;
  }
  .order-md-8 {
    -ms-flex-order: 8;
        order: 8;
  }
  .order-md-9 {
    -ms-flex-order: 9;
        order: 9;
  }
  .order-md-10 {
    -ms-flex-order: 10;
        order: 10;
  }
  .order-md-11 {
    -ms-flex-order: 11;
        order: 11;
  }
  .order-md-12 {
    -ms-flex-order: 12;
        order: 12;
  }
  .offset-md-0 {
    margin-left: 0;
  }
  .offset-md-1 {
    margin-left: 8.333333%;
  }
  .offset-md-2 {
    margin-left: 16.666667%;
  }
  .offset-md-3 {
    margin-left: 25%;
  }
  .offset-md-4 {
    margin-left: 33.333333%;
  }
  .offset-md-5 {
    margin-left: 41.666667%;
  }
  .offset-md-6 {
    margin-left: 50%;
  }
  .offset-md-7 {
    margin-left: 58.333333%;
  }
  .offset-md-8 {
    margin-left: 66.666667%;
  }
  .offset-md-9 {
    margin-left: 75%;
  }
  .offset-md-10 {
    margin-left: 83.333333%;
  }
  .offset-md-11 {
    margin-left: 91.666667%;
  }
}
@media (min-width: 992px) {
  .col-lg {
    -ms-flex-preferred-size: 0;
        flex-basis: 0;
    -ms-flex-positive: 1;
        flex-grow: 1;
    max-width: 100%;
  }
  .col-lg-auto {
    -ms-flex: 0 0 auto;
        flex: 0 0 auto;
    width: auto;
    max-width: none;
  }
  .col-lg-1 {
    -ms-flex: 0 0 8.333333%;
        flex: 0 0 8.333333%;
    max-width: 8.333333%;
  }
  .col-lg-2 {
    -ms-flex: 0 0 16.666667%;
        flex: 0 0 16.666667%;
    max-width: 16.666667%;
  }
  .col-lg-3 {
    -ms-flex: 0 0 25%;
        flex: 0 0 25%;
    max-width: 25%;
  }
  .col-lg-4 {
    -ms-flex: 0 0 33.333333%;
        flex: 0 0 33.333333%;
    max-width: 33.333333%;
  }
  .col-lg-5 {
    -ms-flex: 0 0 41.666667%;
        flex: 0 0 41.666667%;
    max-width: 41.666667%;
  }
  .col-lg-6 {
    -ms-flex: 0 0 50%;
        flex: 0 0 50%;
    max-width: 50%;
  }
  .col-lg-7 {
    -ms-flex: 0 0 58.333333%;
        flex: 0 0 58.333333%;
    max-width: 58.333333%;
  }
  .col-lg-8 {
    -ms-flex: 0 0 66.666667%;
        flex: 0 0 66.666667%;
    max-width: 66.666667%;
  }
  .col-lg-9 {
    -ms-flex: 0 0 75%;
        flex: 0 0 75%;
    max-width: 75%;
  }
  .col-lg-10 {
    -ms-flex: 0 0 83.333333%;
        flex: 0 0 83.333333%;
    max-width: 83.333333%;
  }
  .col-lg-11 {
    -ms-flex: 0 0 91.666667%;
        flex: 0 0 91.666667%;
    max-width: 91.666667%;
  }
  .col-lg-12 {
    -ms-flex: 0 0 100%;
        flex: 0 0 100%;
    max-width: 100%;
  }
  .order-lg-first {
    -ms-flex-order: -1;
        order: -1;
  }
  .order-lg-1 {
    -ms-flex-order: 1;
        order: 1;
  }
  .order-lg-2 {
    -ms-flex-order: 2;
        order: 2;
  }
  .order-lg-3 {
    -ms-flex-order: 3;
        order: 3;
  }
  .order-lg-4 {
    -ms-flex-order: 4;
        order: 4;
  }
  .order-lg-5 {
    -ms-flex-order: 5;
        order: 5;
  }
  .order-lg-6 {
    -ms-flex-order: 6;
        order: 6;
  }
  .order-lg-7 {
    -ms-flex-order: 7;
        order: 7;
  }
  .order-lg-8 {
    -ms-flex-order: 8;
        order: 8;
  }
  .order-lg-9 {
    -ms-flex-order: 9;
        order: 9;
  }
  .order-lg-10 {
    -ms-flex-order: 10;
        order: 10;
  }
  .order-lg-11 {
    -ms-flex-order: 11;
        order: 11;
  }
  .order-lg-12 {
    -ms-flex-order: 12;
        order: 12;
  }
  .offset-lg-0 {
    margin-left: 0;
  }
  .offset-lg-1 {
    margin-left: 8.333333%;
  }
  .offset-lg-2 {
    margin-left: 16.666667%;
  }
  .offset-lg-3 {
    margin-left: 25%;
  }
  .offset-lg-4 {
    margin-left: 33.333333%;
  }
  .offset-lg-5 {
    margin-left: 41.666667%;
  }
  .offset-lg-6 {
    margin-left: 50%;
  }
  .offset-lg-7 {
    margin-left: 58.333333%;
  }
  .offset-lg-8 {
    margin-left: 66.666667%;
  }
  .offset-lg-9 {
    margin-left: 75%;
  }
  .offset-lg-10 {
    margin-left: 83.333333%;
  }
  .offset-lg-11 {
    margin-left: 91.666667%;
  }
}
@media (min-width: 1200px) {
  .col-xl {
    -ms-flex-preferred-size: 0;
        flex-basis: 0;
    -ms-flex-positive: 1;
        flex-grow: 1;
    max-width: 100%;
  }
  .col-xl-auto {
    -ms-flex: 0 0 auto;
        flex: 0 0 auto;
    width: auto;
    max-width: none;
  }
  .col-xl-1 {
    -ms-flex: 0 0 8.333333%;
        flex: 0 0 8.333333%;
    max-width: 8.333333%;
  }
  .col-xl-2 {
    -ms-flex: 0 0 16.666667%;
        flex: 0 0 16.666667%;
    max-width: 16.666667%;
  }
  .col-xl-3 {
    -ms-flex: 0 0 25%;
        flex: 0 0 25%;
    max-width: 25%;
  }
  .col-xl-4 {
    -ms-flex: 0 0 33.333333%;
        flex: 0 0 33.333333%;
    max-width: 33.333333%;
  }
  .col-xl-5 {
    -ms-flex: 0 0 41.666667%;
        flex: 0 0 41.666667%;
    max-width: 41.666667%;
  }
  .col-xl-6 {
    -ms-flex: 0 0 50%;
        flex: 0 0 50%;
    max-width: 50%;
  }
  .col-xl-7 {
    -ms-flex: 0 0 58.333333%;
        flex: 0 0 58.333333%;
    max-width: 58.333333%;
  }
  .col-xl-8 {
    -ms-flex: 0 0 66.666667%;
        flex: 0 0 66.666667%;
    max-width: 66.666667%;
  }
  .col-xl-9 {
    -ms-flex: 0 0 75%;
        flex: 0 0 75%;
    max-width: 75%;
  }
  .col-xl-10 {
    -ms-flex: 0 0 83.333333%;
        flex: 0 0 83.333333%;
    max-width: 83.333333%;
  }
  .col-xl-11 {
    -ms-flex: 0 0 91.666667%;
        flex: 0 0 91.666667%;
    max-width: 91.666667%;
  }
  .col-xl-12 {
    -ms-flex: 0 0 100%;
        flex: 0 0 100%;
    max-width: 100%;
  }
  .order-xl-first {
    -ms-flex-order: -1;
        order: -1;
  }
  .order-xl-1 {
    -ms-flex-order: 1;
        order: 1;
  }
  .order-xl-2 {
    -ms-flex-order: 2;
        order: 2;
  }
  .order-xl-3 {
    -ms-flex-order: 3;
        order: 3;
  }
  .order-xl-4 {
    -ms-flex-order: 4;
        order: 4;
  }
  .order-xl-5 {
    -ms-flex-order: 5;
        order: 5;
  }
  .order-xl-6 {
    -ms-flex-order: 6;
        order: 6;
  }
  .order-xl-7 {
    -ms-flex-order: 7;
        order: 7;
  }
  .order-xl-8 {
    -ms-flex-order: 8;
        order: 8;
  }
  .order-xl-9 {
    -ms-flex-order: 9;
        order: 9;
  }
  .order-xl-10 {
    -ms-flex-order: 10;
        order: 10;
  }
  .order-xl-11 {
    -ms-flex-order: 11;
        order: 11;
  }
  .order-xl-12 {
    -ms-flex-order: 12;
        order: 12;
  }
  .offset-xl-0 {
    margin-left: 0;
  }
  .offset-xl-1 {
    margin-left: 8.333333%;
  }
  .offset-xl-2 {
    margin-left: 16.666667%;
  }
  .offset-xl-3 {
    margin-left: 25%;
  }
  .offset-xl-4 {
    margin-left: 33.333333%;
  }
  .offset-xl-5 {
    margin-left: 41.666667%;
  }
  .offset-xl-6 {
    margin-left: 50%;
  }
  .offset-xl-7 {
    margin-left: 58.333333%;
  }
  .offset-xl-8 {
    margin-left: 66.666667%;
  }
  .offset-xl-9 {
    margin-left: 75%;
  }
  .offset-xl-10 {
    margin-left: 83.333333%;
  }
  .offset-xl-11 {
    margin-left: 91.666667%;
  }
}
.table {
  width: 100%;
  max-width: 100%;
  margin-bottom: 1rem;
  background-color: transparent;
}
.table th,
.table td {
  padding: 0.75rem;
  vertical-align: top;
  border-top: 1px solid #e9ecef;
}
.table thead th {
  vertical-align: bottom;
  border-bottom: 2px solid #e9ecef;
}
.table tbody + tbody {
  border-top: 2px solid #e9ecef;
}
.table .table {
  background-color: #fff;
}
.table-sm th,
.table-sm td {
  padding: 0.3rem;
}
.table-bordered {
  border: 1px solid #e9ecef;
}
.table-bordered th,
.table-bordered td {
  border: 1px solid #e9ecef;
}
.table-bordered thead th,
.table-bordered thead td {
  border-bottom-width: 2px;
}
.table-striped tbody tr:nth-of-type(odd) {
  background-color: rgba(0, 0, 0, 0.05);
}
.table-hover tbody tr:hover {
  background-color: rgba(0, 0, 0, 0.075);
}
.table-primary,
.table-primary > th,
.table-primary > td {
  background-color: #b8daff;
}
.table-hover .table-primary:hover {
  background-color: #9fcdff;
}
.table-hover .table-primary:hover > td,
.table-hover .table-primary:hover > th {
  background-color: #9fcdff;
}
.table-secondary,
.table-secondary > th,
.table-secondary > td {
  background-color: #dddfe2;
}
.table-hover .table-secondary:hover {
  background-color: #cfd2d6;
}
.table-hover .table-secondary:hover > td,
.table-hover .table-secondary:hover > th {
  background-color: #cfd2d6;
}
.table-success,
.table-success > th,
.table-success > td {
  background-color: #c3e6cb;
}
.table-hover .table-success:hover {
  background-color: #b1dfbb;
}
.table-hover .table-success:hover > td,
.table-hover .table-success:hover > th {
  background-color: #b1dfbb;
}
.table-info,
.table-info > th,
.table-info > td {
  background-color: #bee5eb;
}
.table-hover .table-info:hover {
  background-color: #abdde5;
}
.table-hover .table-info:hover > td,
.table-hover .table-info:hover > th {
  background-color: #abdde5;
}
.table-warning,
.table-warning > th,
.table-warning > td {
  background-color: #ffeeba;
}
.table-hover .table-warning:hover {
  background-color: #ffe8a1;
}
.table-hover .table-warning:hover > td,
.table-hover .table-warning:hover > th {
  background-color: #ffe8a1;
}
.table-danger,
.table-danger > th,
.table-danger > td {
  background-color: #f5c6cb;
}
.table-hover .table-danger:hover {
  background-color: #f1b0b7;
}
.table-hover .table-danger:hover > td,
.table-hover .table-danger:hover > th {
  background-color: #f1b0b7;
}
.table-light,
.table-light > th,
.table-light > td {
  background-color: #fdfdfe;
}
.table-hover .table-light:hover {
  background-color: #ececf6;
}
.table-hover .table-light:hover > td,
.table-hover .table-light:hover > th {
  background-color: #ececf6;
}
.table-dark,
.table-dark > th,
.table-dark > td {
  background-color: #c6c8ca;
}
.table-hover .table-dark:hover {
  background-color: #b9bbbe;
}
.table-hover .table-dark:hover > td,
.table-hover .table-dark:hover > th {
  background-color: #b9bbbe;
}
.table-active,
.table-active > th,
.table-active > td {
  background-color: rgba(0, 0, 0, 0.075);
}
.table-hover .table-active:hover {
  background-color: rgba(0, 0, 0, 0.075);
}
.table-hover .table-active:hover > td,
.table-hover .table-active:hover > th {
  background-color: rgba(0, 0, 0, 0.075);
}
.table .thead-dark th {
  color: #fff;
  background-color: #212529;
  border-color: #32383e;
}
.table .thead-light th {
  color: #495057;
  background-color: #e9ecef;
  border-color: #e9ecef;
}
.table-dark {
  color: #fff;
  background-color: #212529;
}
.table-dark th,
.table-dark td,
.table-dark thead th {
  border-color: #32383e;
}
.table-dark.table-bordered {
  border: 0;
}
.table-dark.table-striped tbody tr:nth-of-type(odd) {
  background-color: rgba(255, 255, 255, 0.05);
}
.table-dark.table-hover tbody tr:hover {
  background-color: rgba(255, 255, 255, 0.075);
}
@media (max-width: 575px) {
  .table-responsive-sm {
    display: block;
    width: 100%;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    -ms-overflow-style: -ms-autohiding-scrollbar;
  }
  .table-responsive-sm.table-bordered {
    border: 0;
  }
}
@media (max-width: 767px) {
  .table-responsive-md {
    display: block;
    width: 100%;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    -ms-overflow-style: -ms-autohiding-scrollbar;
  }
  .table-responsive-md.table-bordered {
    border: 0;
  }
}
@media (max-width: 991px) {
  .table-responsive-lg {
    display: block;
    width: 100%;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    -ms-overflow-style: -ms-autohiding-scrollbar;
  }
  .table-responsive-lg.table-bordered {
    border: 0;
  }
}
@media (max-width: 1199px) {
  .table-responsive-xl {
    display: block;
    width: 100%;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    -ms-overflow-style: -ms-autohiding-scrollbar;
  }
  .table-responsive-xl.table-bordered {
    border: 0;
  }
}
.table-responsive {
  display: block;
  width: 100%;
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
  -ms-overflow-style: -ms-autohiding-scrollbar;
}
.table-responsive.table-bordered {
  border: 0;
}
.form-control {
  display: block;
  width: 100%;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  color: #495057;
  background-color: #fff;
  background-image: none;
  background-clip: padding-box;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
  transition: border-color ease-in-out 0.15s, box-shadow ease-in-out 0.15s;
}
.form-control::-ms-expand {
  background-color: transparent;
  border: 0;
}
.form-control:focus {
  color: #495057;
  background-color: #fff;
  border-color: #80bdff;
  outline: none;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}
.form-control::-webkit-input-placeholder {
  color: #868e96;
  opacity: 1;
}
.form-control:-ms-input-placeholder {
  color: #868e96;
  opacity: 1;
}
.form-control::-ms-input-placeholder {
  color: #868e96;
  opacity: 1;
}
.form-control::placeholder {
  color: #868e96;
  opacity: 1;
}
.form-control:disabled, .form-control[readonly] {
  background-color: #e9ecef;
  opacity: 1;
}
select.form-control:not([size]):not([multiple]) {
  height: calc(2.25rem + 2px);
}
select.form-control:focus::-ms-value {
  color: #495057;
  background-color: #fff;
}
.form-control-file,
.form-control-range {
  display: block;
}
.col-form-label {
  padding-top: calc(0.375rem + 1px);
  padding-bottom: calc(0.375rem + 1px);
  margin-bottom: 0;
  line-height: 1.5;
}
.col-form-label-lg {
  padding-top: calc(0.5rem + 1px);
  padding-bottom: calc(0.5rem + 1px);
  font-size: 1.25rem;
  line-height: 1.5;
}
.col-form-label-sm {
  padding-top: calc(0.25rem + 1px);
  padding-bottom: calc(0.25rem + 1px);
  font-size: 0.875rem;
  line-height: 1.5;
}
.col-form-legend {
  padding-top: 0.375rem;
  padding-bottom: 0.375rem;
  margin-bottom: 0;
  font-size: 1rem;
}
.form-control-plaintext {
  padding-top: 0.375rem;
  padding-bottom: 0.375rem;
  margin-bottom: 0;
  line-height: 1.5;
  background-color: transparent;
  border: solid transparent;
  border-width: 1px 0;
}
.form-control-plaintext.form-control-sm, .input-group-sm > .form-control-plaintext.form-control,
.input-group-sm > .form-control-plaintext.input-group-addon,
.input-group-sm > .input-group-btn > .form-control-plaintext.btn, .form-control-plaintext.form-control-lg, .input-group-lg > .form-control-plaintext.form-control,
.input-group-lg > .form-control-plaintext.input-group-addon,
.input-group-lg > .input-group-btn > .form-control-plaintext.btn {
  padding-right: 0;
  padding-left: 0;
}
.form-control-sm, .input-group-sm > .form-control,
.input-group-sm > .input-group-addon,
.input-group-sm > .input-group-btn > .btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  line-height: 1.5;
  border-radius: 0.2rem;
}
select.form-control-sm:not([size]):not([multiple]), .input-group-sm > select.form-control:not([size]):not([multiple]),
.input-group-sm > select.input-group-addon:not([size]):not([multiple]),
.input-group-sm > .input-group-btn > select.btn:not([size]):not([multiple]) {
  height: calc(1.8125rem + 2px);
}
.form-control-lg, .input-group-lg > .form-control,
.input-group-lg > .input-group-addon,
.input-group-lg > .input-group-btn > .btn {
  padding: 0.5rem 1rem;
  font-size: 1.25rem;
  line-height: 1.5;
  border-radius: 0.3rem;
}
select.form-control-lg:not([size]):not([multiple]), .input-group-lg > select.form-control:not([size]):not([multiple]),
.input-group-lg > select.input-group-addon:not([size]):not([multiple]),
.input-group-lg > .input-group-btn > select.btn:not([size]):not([multiple]) {
  height: calc(2.875rem + 2px);
}
.form-group {
  margin-bottom: 1rem;
}
.form-text {
  display: block;
  margin-top: 0.25rem;
}
.form-row {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  margin-right: -5px;
  margin-left: -5px;
}
.form-row > .col,
.form-row > [class*=&quot;col-&quot;] {
  padding-right: 5px;
  padding-left: 5px;
}
.form-check {
  position: relative;
  display: block;
  margin-bottom: 0.5rem;
}
.form-check.disabled .form-check-label {
  color: #868e96;
}
.form-check-label {
  padding-left: 1.25rem;
  margin-bottom: 0;
}
.form-check-input {
  position: absolute;
  margin-top: 0.25rem;
  margin-left: -1.25rem;
}
.form-check-inline {
  display: inline-block;
  margin-right: 0.75rem;
}
.form-check-inline .form-check-label {
  vertical-align: middle;
}
.valid-feedback {
  display: none;
  margin-top: .25rem;
  font-size: .875rem;
  color: #28a745;
}
.valid-tooltip {
  position: absolute;
  top: 100%;
  z-index: 5;
  display: none;
  width: 250px;
  padding: .5rem;
  margin-top: .1rem;
  font-size: .875rem;
  line-height: 1;
  color: #fff;
  background-color: rgba(40, 167, 69, 0.8);
  border-radius: .2rem;
}
.was-validated .form-control:valid, .form-control.is-valid, .was-validated
.custom-select:valid,
.custom-select.is-valid {
  border-color: #28a745;
}
.was-validated .form-control:valid:focus, .form-control.is-valid:focus, .was-validated
.custom-select:valid:focus,
.custom-select.is-valid:focus {
  box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.25);
}
.was-validated .form-control:valid ~ .valid-feedback,
.was-validated .form-control:valid ~ .valid-tooltip, .form-control.is-valid ~ .valid-feedback,
.form-control.is-valid ~ .valid-tooltip, .was-validated
.custom-select:valid ~ .valid-feedback,
.was-validated
.custom-select:valid ~ .valid-tooltip,
.custom-select.is-valid ~ .valid-feedback,
.custom-select.is-valid ~ .valid-tooltip {
  display: block;
}
.was-validated .form-check-input:valid + .form-check-label, .form-check-input.is-valid + .form-check-label {
  color: #28a745;
}
.was-validated .custom-control-input:valid ~ .custom-control-indicator, .custom-control-input.is-valid ~ .custom-control-indicator {
  background-color: rgba(40, 167, 69, 0.25);
}
.was-validated .custom-control-input:valid ~ .custom-control-description, .custom-control-input.is-valid ~ .custom-control-description {
  color: #28a745;
}
.was-validated .custom-file-input:valid ~ .custom-file-control, .custom-file-input.is-valid ~ .custom-file-control {
  border-color: #28a745;
}
.was-validated .custom-file-input:valid ~ .custom-file-control::before, .custom-file-input.is-valid ~ .custom-file-control::before {
  border-color: inherit;
}
.was-validated .custom-file-input:valid:focus, .custom-file-input.is-valid:focus {
  box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.25);
}
.invalid-feedback {
  display: none;
  margin-top: .25rem;
  font-size: .875rem;
  color: #dc3545;
}
.invalid-tooltip {
  position: absolute;
  top: 100%;
  z-index: 5;
  display: none;
  width: 250px;
  padding: .5rem;
  margin-top: .1rem;
  font-size: .875rem;
  line-height: 1;
  color: #fff;
  background-color: rgba(220, 53, 69, 0.8);
  border-radius: .2rem;
}
.was-validated .form-control:invalid, .form-control.is-invalid, .was-validated
.custom-select:invalid,
.custom-select.is-invalid {
  border-color: #dc3545;
}
.was-validated .form-control:invalid:focus, .form-control.is-invalid:focus, .was-validated
.custom-select:invalid:focus,
.custom-select.is-invalid:focus {
  box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.25);
}
.was-validated .form-control:invalid ~ .invalid-feedback,
.was-validated .form-control:invalid ~ .invalid-tooltip, .form-control.is-invalid ~ .invalid-feedback,
.form-control.is-invalid ~ .invalid-tooltip, .was-validated
.custom-select:invalid ~ .invalid-feedback,
.was-validated
.custom-select:invalid ~ .invalid-tooltip,
.custom-select.is-invalid ~ .invalid-feedback,
.custom-select.is-invalid ~ .invalid-tooltip {
  display: block;
}
.was-validated .form-check-input:invalid + .form-check-label, .form-check-input.is-invalid + .form-check-label {
  color: #dc3545;
}
.was-validated .custom-control-input:invalid ~ .custom-control-indicator, .custom-control-input.is-invalid ~ .custom-control-indicator {
  background-color: rgba(220, 53, 69, 0.25);
}
.was-validated .custom-control-input:invalid ~ .custom-control-description, .custom-control-input.is-invalid ~ .custom-control-description {
  color: #dc3545;
}
.was-validated .custom-file-input:invalid ~ .custom-file-control, .custom-file-input.is-invalid ~ .custom-file-control {
  border-color: #dc3545;
}
.was-validated .custom-file-input:invalid ~ .custom-file-control::before, .custom-file-input.is-invalid ~ .custom-file-control::before {
  border-color: inherit;
}
.was-validated .custom-file-input:invalid:focus, .custom-file-input.is-invalid:focus {
  box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.25);
}
.form-inline {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-flow: row wrap;
      flex-flow: row wrap;
  -ms-flex-align: center;
      align-items: center;
}
.form-inline .form-check {
  width: 100%;
}
@media (min-width: 576px) {
  .form-inline label {
    display: -ms-flexbox;
    display: flex;
    -ms-flex-align: center;
        align-items: center;
    -ms-flex-pack: center;
        justify-content: center;
    margin-bottom: 0;
  }
  .form-inline .form-group {
    display: -ms-flexbox;
    display: flex;
    -ms-flex: 0 0 auto;
        flex: 0 0 auto;
    -ms-flex-flow: row wrap;
        flex-flow: row wrap;
    -ms-flex-align: center;
        align-items: center;
    margin-bottom: 0;
  }
  .form-inline .form-control {
    display: inline-block;
    width: auto;
    vertical-align: middle;
  }
  .form-inline .form-control-plaintext {
    display: inline-block;
  }
  .form-inline .input-group {
    width: auto;
  }
  .form-inline .form-check {
    display: -ms-flexbox;
    display: flex;
    -ms-flex-align: center;
        align-items: center;
    -ms-flex-pack: center;
        justify-content: center;
    width: auto;
    margin-top: 0;
    margin-bottom: 0;
  }
  .form-inline .form-check-label {
    padding-left: 0;
  }
  .form-inline .form-check-input {
    position: relative;
    margin-top: 0;
    margin-right: 0.25rem;
    margin-left: 0;
  }
  .form-inline .custom-control {
    display: -ms-flexbox;
    display: flex;
    -ms-flex-align: center;
        align-items: center;
    -ms-flex-pack: center;
        justify-content: center;
    padding-left: 0;
  }
  .form-inline .custom-control-indicator {
    position: static;
    display: inline-block;
    margin-right: 0.25rem;
    vertical-align: text-bottom;
  }
  .form-inline .has-feedback .form-control-feedback {
    top: 0;
  }
}
.btn {
  display: inline-block;
  font-weight: 400;
  text-align: center;
  white-space: nowrap;
  vertical-align: middle;
  -webkit-user-select: none;
     -moz-user-select: none;
      -ms-user-select: none;
          user-select: none;
  border: 1px solid transparent;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  border-radius: 0.25rem;
  transition: background-color 0.15s ease-in-out, border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}
.btn:focus, .btn:hover {
  text-decoration: none;
}
.btn:focus, .btn.focus {
  outline: 0;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}
.btn.disabled, .btn:disabled {
  opacity: .65;
}
.btn:not([disabled]):not(.disabled):active, .btn:not([disabled]):not(.disabled).active {
  background-image: none;
}
a.btn.disabled,
fieldset[disabled] a.btn {
  pointer-events: none;
}
.btn-primary {
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
}
.btn-primary:hover {
  color: #fff;
  background-color: #0069d9;
  border-color: #0062cc;
}
.btn-primary:focus, .btn-primary.focus {
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.5);
}
.btn-primary.disabled, .btn-primary:disabled {
  background-color: #007bff;
  border-color: #007bff;
}
.btn-primary:not([disabled]):not(.disabled):active, .btn-primary:not([disabled]):not(.disabled).active,
.show > .btn-primary.dropdown-toggle {
  color: #fff;
  background-color: #0062cc;
  border-color: #005cbf;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.5);
}
.btn-secondary {
  color: #fff;
  background-color: #868e96;
  border-color: #868e96;
}
.btn-secondary:hover {
  color: #fff;
  background-color: #727b84;
  border-color: #6c757d;
}
.btn-secondary:focus, .btn-secondary.focus {
  box-shadow: 0 0 0 0.2rem rgba(134, 142, 150, 0.5);
}
.btn-secondary.disabled, .btn-secondary:disabled {
  background-color: #868e96;
  border-color: #868e96;
}
.btn-secondary:not([disabled]):not(.disabled):active, .btn-secondary:not([disabled]):not(.disabled).active,
.show > .btn-secondary.dropdown-toggle {
  color: #fff;
  background-color: #6c757d;
  border-color: #666e76;
  box-shadow: 0 0 0 0.2rem rgba(134, 142, 150, 0.5);
}
.btn-success {
  color: #fff;
  background-color: #28a745;
  border-color: #28a745;
}
.btn-success:hover {
  color: #fff;
  background-color: #218838;
  border-color: #1e7e34;
}
.btn-success:focus, .btn-success.focus {
  box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.5);
}
.btn-success.disabled, .btn-success:disabled {
  background-color: #28a745;
  border-color: #28a745;
}
.btn-success:not([disabled]):not(.disabled):active, .btn-success:not([disabled]):not(.disabled).active,
.show > .btn-success.dropdown-toggle {
  color: #fff;
  background-color: #1e7e34;
  border-color: #1c7430;
  box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.5);
}
.btn-info {
  color: #fff;
  background-color: #17a2b8;
  border-color: #17a2b8;
}
.btn-info:hover {
  color: #fff;
  background-color: #138496;
  border-color: #117a8b;
}
.btn-info:focus, .btn-info.focus {
  box-shadow: 0 0 0 0.2rem rgba(23, 162, 184, 0.5);
}
.btn-info.disabled, .btn-info:disabled {
  background-color: #17a2b8;
  border-color: #17a2b8;
}
.btn-info:not([disabled]):not(.disabled):active, .btn-info:not([disabled]):not(.disabled).active,
.show > .btn-info.dropdown-toggle {
  color: #fff;
  background-color: #117a8b;
  border-color: #10707f;
  box-shadow: 0 0 0 0.2rem rgba(23, 162, 184, 0.5);
}
.btn-warning {
  color: #111;
  background-color: #ffc107;
  border-color: #ffc107;
}
.btn-warning:hover {
  color: #111;
  background-color: #e0a800;
  border-color: #d39e00;
}
.btn-warning:focus, .btn-warning.focus {
  box-shadow: 0 0 0 0.2rem rgba(255, 193, 7, 0.5);
}
.btn-warning.disabled, .btn-warning:disabled {
  background-color: #ffc107;
  border-color: #ffc107;
}
.btn-warning:not([disabled]):not(.disabled):active, .btn-warning:not([disabled]):not(.disabled).active,
.show > .btn-warning.dropdown-toggle {
  color: #111;
  background-color: #d39e00;
  border-color: #c69500;
  box-shadow: 0 0 0 0.2rem rgba(255, 193, 7, 0.5);
}
.btn-danger {
  color: #fff;
  background-color: #dc3545;
  border-color: #dc3545;
}
.btn-danger:hover {
  color: #fff;
  background-color: #c82333;
  border-color: #bd2130;
}
.btn-danger:focus, .btn-danger.focus {
  box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.5);
}
.btn-danger.disabled, .btn-danger:disabled {
  background-color: #dc3545;
  border-color: #dc3545;
}
.btn-danger:not([disabled]):not(.disabled):active, .btn-danger:not([disabled]):not(.disabled).active,
.show > .btn-danger.dropdown-toggle {
  color: #fff;
  background-color: #bd2130;
  border-color: #b21f2d;
  box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.5);
}
.btn-light {
  color: #111;
  background-color: #f8f9fa;
  border-color: #f8f9fa;
}
.btn-light:hover {
  color: #111;
  background-color: #e2e6ea;
  border-color: #dae0e5;
}
.btn-light:focus, .btn-light.focus {
  box-shadow: 0 0 0 0.2rem rgba(248, 249, 250, 0.5);
}
.btn-light.disabled, .btn-light:disabled {
  background-color: #f8f9fa;
  border-color: #f8f9fa;
}
.btn-light:not([disabled]):not(.disabled):active, .btn-light:not([disabled]):not(.disabled).active,
.show > .btn-light.dropdown-toggle {
  color: #111;
  background-color: #dae0e5;
  border-color: #d3d9df;
  box-shadow: 0 0 0 0.2rem rgba(248, 249, 250, 0.5);
}
.btn-dark {
  color: #fff;
  background-color: #343a40;
  border-color: #343a40;
}
.btn-dark:hover {
  color: #fff;
  background-color: #23272b;
  border-color: #1d2124;
}
.btn-dark:focus, .btn-dark.focus {
  box-shadow: 0 0 0 0.2rem rgba(52, 58, 64, 0.5);
}
.btn-dark.disabled, .btn-dark:disabled {
  background-color: #343a40;
  border-color: #343a40;
}
.btn-dark:not([disabled]):not(.disabled):active, .btn-dark:not([disabled]):not(.disabled).active,
.show > .btn-dark.dropdown-toggle {
  color: #fff;
  background-color: #1d2124;
  border-color: #171a1d;
  box-shadow: 0 0 0 0.2rem rgba(52, 58, 64, 0.5);
}
.btn-outline-primary {
  color: #007bff;
  background-color: transparent;
  background-image: none;
  border-color: #007bff;
}
.btn-outline-primary:hover {
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
}
.btn-outline-primary:focus, .btn-outline-primary.focus {
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.5);
}
.btn-outline-primary.disabled, .btn-outline-primary:disabled {
  color: #007bff;
  background-color: transparent;
}
.btn-outline-primary:not([disabled]):not(.disabled):active, .btn-outline-primary:not([disabled]):not(.disabled).active,
.show > .btn-outline-primary.dropdown-toggle {
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.5);
}
.btn-outline-secondary {
  color: #868e96;
  background-color: transparent;
  background-image: none;
  border-color: #868e96;
}
.btn-outline-secondary:hover {
  color: #fff;
  background-color: #868e96;
  border-color: #868e96;
}
.btn-outline-secondary:focus, .btn-outline-secondary.focus {
  box-shadow: 0 0 0 0.2rem rgba(134, 142, 150, 0.5);
}
.btn-outline-secondary.disabled, .btn-outline-secondary:disabled {
  color: #868e96;
  background-color: transparent;
}
.btn-outline-secondary:not([disabled]):not(.disabled):active, .btn-outline-secondary:not([disabled]):not(.disabled).active,
.show > .btn-outline-secondary.dropdown-toggle {
  color: #fff;
  background-color: #868e96;
  border-color: #868e96;
  box-shadow: 0 0 0 0.2rem rgba(134, 142, 150, 0.5);
}
.btn-outline-success {
  color: #28a745;
  background-color: transparent;
  background-image: none;
  border-color: #28a745;
}
.btn-outline-success:hover {
  color: #fff;
  background-color: #28a745;
  border-color: #28a745;
}
.btn-outline-success:focus, .btn-outline-success.focus {
  box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.5);
}
.btn-outline-success.disabled, .btn-outline-success:disabled {
  color: #28a745;
  background-color: transparent;
}
.btn-outline-success:not([disabled]):not(.disabled):active, .btn-outline-success:not([disabled]):not(.disabled).active,
.show > .btn-outline-success.dropdown-toggle {
  color: #fff;
  background-color: #28a745;
  border-color: #28a745;
  box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.5);
}
.btn-outline-info {
  color: #17a2b8;
  background-color: transparent;
  background-image: none;
  border-color: #17a2b8;
}
.btn-outline-info:hover {
  color: #fff;
  background-color: #17a2b8;
  border-color: #17a2b8;
}
.btn-outline-info:focus, .btn-outline-info.focus {
  box-shadow: 0 0 0 0.2rem rgba(23, 162, 184, 0.5);
}
.btn-outline-info.disabled, .btn-outline-info:disabled {
  color: #17a2b8;
  background-color: transparent;
}
.btn-outline-info:not([disabled]):not(.disabled):active, .btn-outline-info:not([disabled]):not(.disabled).active,
.show > .btn-outline-info.dropdown-toggle {
  color: #fff;
  background-color: #17a2b8;
  border-color: #17a2b8;
  box-shadow: 0 0 0 0.2rem rgba(23, 162, 184, 0.5);
}
.btn-outline-warning {
  color: #ffc107;
  background-color: transparent;
  background-image: none;
  border-color: #ffc107;
}
.btn-outline-warning:hover {
  color: #fff;
  background-color: #ffc107;
  border-color: #ffc107;
}
.btn-outline-warning:focus, .btn-outline-warning.focus {
  box-shadow: 0 0 0 0.2rem rgba(255, 193, 7, 0.5);
}
.btn-outline-warning.disabled, .btn-outline-warning:disabled {
  color: #ffc107;
  background-color: transparent;
}
.btn-outline-warning:not([disabled]):not(.disabled):active, .btn-outline-warning:not([disabled]):not(.disabled).active,
.show > .btn-outline-warning.dropdown-toggle {
  color: #fff;
  background-color: #ffc107;
  border-color: #ffc107;
  box-shadow: 0 0 0 0.2rem rgba(255, 193, 7, 0.5);
}
.btn-outline-danger {
  color: #dc3545;
  background-color: transparent;
  background-image: none;
  border-color: #dc3545;
}
.btn-outline-danger:hover {
  color: #fff;
  background-color: #dc3545;
  border-color: #dc3545;
}
.btn-outline-danger:focus, .btn-outline-danger.focus {
  box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.5);
}
.btn-outline-danger.disabled, .btn-outline-danger:disabled {
  color: #dc3545;
  background-color: transparent;
}
.btn-outline-danger:not([disabled]):not(.disabled):active, .btn-outline-danger:not([disabled]):not(.disabled).active,
.show > .btn-outline-danger.dropdown-toggle {
  color: #fff;
  background-color: #dc3545;
  border-color: #dc3545;
  box-shadow: 0 0 0 0.2rem rgba(220, 53, 69, 0.5);
}
.btn-outline-light {
  color: #f8f9fa;
  background-color: transparent;
  background-image: none;
  border-color: #f8f9fa;
}
.btn-outline-light:hover {
  color: #212529;
  background-color: #f8f9fa;
  border-color: #f8f9fa;
}
.btn-outline-light:focus, .btn-outline-light.focus {
  box-shadow: 0 0 0 0.2rem rgba(248, 249, 250, 0.5);
}
.btn-outline-light.disabled, .btn-outline-light:disabled {
  color: #f8f9fa;
  background-color: transparent;
}
.btn-outline-light:not([disabled]):not(.disabled):active, .btn-outline-light:not([disabled]):not(.disabled).active,
.show > .btn-outline-light.dropdown-toggle {
  color: #212529;
  background-color: #f8f9fa;
  border-color: #f8f9fa;
  box-shadow: 0 0 0 0.2rem rgba(248, 249, 250, 0.5);
}
.btn-outline-dark {
  color: #343a40;
  background-color: transparent;
  background-image: none;
  border-color: #343a40;
}
.btn-outline-dark:hover {
  color: #fff;
  background-color: #343a40;
  border-color: #343a40;
}
.btn-outline-dark:focus, .btn-outline-dark.focus {
  box-shadow: 0 0 0 0.2rem rgba(52, 58, 64, 0.5);
}
.btn-outline-dark.disabled, .btn-outline-dark:disabled {
  color: #343a40;
  background-color: transparent;
}
.btn-outline-dark:not([disabled]):not(.disabled):active, .btn-outline-dark:not([disabled]):not(.disabled).active,
.show > .btn-outline-dark.dropdown-toggle {
  color: #fff;
  background-color: #343a40;
  border-color: #343a40;
  box-shadow: 0 0 0 0.2rem rgba(52, 58, 64, 0.5);
}
.btn-link {
  font-weight: 400;
  color: #007bff;
  background-color: transparent;
}
.btn-link:hover {
  color: #0056b3;
  text-decoration: underline;
  background-color: transparent;
  border-color: transparent;
}
.btn-link:focus, .btn-link.focus {
  border-color: transparent;
  box-shadow: none;
}
.btn-link:disabled, .btn-link.disabled {
  color: #868e96;
}
.btn-lg, .btn-group-lg > .btn {
  padding: 0.5rem 1rem;
  font-size: 1.25rem;
  line-height: 1.5;
  border-radius: 0.3rem;
}
.btn-sm, .btn-group-sm > .btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  line-height: 1.5;
  border-radius: 0.2rem;
}
.btn-block {
  display: block;
  width: 100%;
}
.btn-block + .btn-block {
  margin-top: 0.5rem;
}
input[type=&quot;submit&quot;].btn-block,
input[type=&quot;reset&quot;].btn-block,
input[type=&quot;button&quot;].btn-block {
  width: 100%;
}
.fade {
  opacity: 0;
  transition: opacity 0.15s linear;
}
.fade.show {
  opacity: 1;
}
.collapse {
  display: none;
}
.collapse.show {
  display: block;
}
tr.collapse.show {
  display: table-row;
}
tbody.collapse.show {
  display: table-row-group;
}
.collapsing {
  position: relative;
  height: 0;
  overflow: hidden;
  transition: height 0.35s ease;
}
.dropup,
.dropdown {
  position: relative;
}
.dropdown-toggle::after {
  display: inline-block;
  width: 0;
  height: 0;
  margin-left: 0.255em;
  vertical-align: 0.255em;
  content: &quot;&quot;;
  border-top: 0.3em solid;
  border-right: 0.3em solid transparent;
  border-bottom: 0;
  border-left: 0.3em solid transparent;
}
.dropdown-toggle:empty::after {
  margin-left: 0;
}
.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 1000;
  display: none;
  float: left;
  min-width: 10rem;
  padding: 0.5rem 0;
  margin: 0.125rem 0 0;
  font-size: 1rem;
  color: #212529;
  text-align: left;
  list-style: none;
  background-color: #fff;
  background-clip: padding-box;
  border: 1px solid rgba(0, 0, 0, 0.15);
  border-radius: 0.25rem;
}
.dropup .dropdown-menu {
  margin-top: 0;
  margin-bottom: 0.125rem;
}
.dropup .dropdown-toggle::after {
  display: inline-block;
  width: 0;
  height: 0;
  margin-left: 0.255em;
  vertical-align: 0.255em;
  content: &quot;&quot;;
  border-top: 0;
  border-right: 0.3em solid transparent;
  border-bottom: 0.3em solid;
  border-left: 0.3em solid transparent;
}
.dropup .dropdown-toggle:empty::after {
  margin-left: 0;
}
.dropdown-divider {
  height: 0;
  margin: 0.5rem 0;
  overflow: hidden;
  border-top: 1px solid #e9ecef;
}
.dropdown-item {
  display: block;
  width: 100%;
  padding: 0.25rem 1.5rem;
  clear: both;
  font-weight: 400;
  color: #212529;
  text-align: inherit;
  white-space: nowrap;
  background: none;
  border: 0;
}
.dropdown-item:focus, .dropdown-item:hover {
  color: #16181b;
  text-decoration: none;
  background-color: #f8f9fa;
}
.dropdown-item.active, .dropdown-item:active {
  color: #fff;
  text-decoration: none;
  background-color: #007bff;
}
.dropdown-item.disabled, .dropdown-item:disabled {
  color: #868e96;
  background-color: transparent;
}
.dropdown-menu.show {
  display: block;
}
.dropdown-header {
  display: block;
  padding: 0.5rem 1.5rem;
  margin-bottom: 0;
  font-size: 0.875rem;
  color: #868e96;
  white-space: nowrap;
}
.btn-group,
.btn-group-vertical {
  position: relative;
  display: -ms-inline-flexbox;
  display: inline-flex;
  vertical-align: middle;
}
.btn-group > .btn,
.btn-group-vertical > .btn {
  position: relative;
  -ms-flex: 0 1 auto;
      flex: 0 1 auto;
}
.btn-group > .btn:hover,
.btn-group-vertical > .btn:hover {
  z-index: 2;
}
.btn-group > .btn:focus, .btn-group > .btn:active, .btn-group > .btn.active,
.btn-group-vertical > .btn:focus,
.btn-group-vertical > .btn:active,
.btn-group-vertical > .btn.active {
  z-index: 2;
}
.btn-group .btn + .btn,
.btn-group .btn + .btn-group,
.btn-group .btn-group + .btn,
.btn-group .btn-group + .btn-group,
.btn-group-vertical .btn + .btn,
.btn-group-vertical .btn + .btn-group,
.btn-group-vertical .btn-group + .btn,
.btn-group-vertical .btn-group + .btn-group {
  margin-left: -1px;
}
.btn-toolbar {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  -ms-flex-pack: start;
      justify-content: flex-start;
}
.btn-toolbar .input-group {
  width: auto;
}
.btn-group > .btn:not(:first-child):not(:last-child):not(.dropdown-toggle) {
  border-radius: 0;
}
.btn-group > .btn:first-child {
  margin-left: 0;
}
.btn-group > .btn:first-child:not(:last-child):not(.dropdown-toggle) {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}
.btn-group > .btn:last-child:not(:first-child),
.btn-group > .dropdown-toggle:not(:first-child) {
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}
.btn-group > .btn-group {
  float: left;
}
.btn-group > .btn-group:not(:first-child):not(:last-child) > .btn {
  border-radius: 0;
}
.btn-group > .btn-group:first-child:not(:last-child) > .btn:last-child,
.btn-group > .btn-group:first-child:not(:last-child) > .dropdown-toggle {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}
.btn-group > .btn-group:last-child:not(:first-child) > .btn:first-child {
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}
.btn + .dropdown-toggle-split {
  padding-right: 0.5625rem;
  padding-left: 0.5625rem;
}
.btn + .dropdown-toggle-split::after {
  margin-left: 0;
}
.btn-sm + .dropdown-toggle-split, .btn-group-sm > .btn + .dropdown-toggle-split {
  padding-right: 0.375rem;
  padding-left: 0.375rem;
}
.btn-lg + .dropdown-toggle-split, .btn-group-lg > .btn + .dropdown-toggle-split {
  padding-right: 0.75rem;
  padding-left: 0.75rem;
}
.btn-group-vertical {
  -ms-flex-direction: column;
      flex-direction: column;
  -ms-flex-align: start;
      align-items: flex-start;
  -ms-flex-pack: center;
      justify-content: center;
}
.btn-group-vertical .btn,
.btn-group-vertical .btn-group {
  width: 100%;
}
.btn-group-vertical > .btn + .btn,
.btn-group-vertical > .btn + .btn-group,
.btn-group-vertical > .btn-group + .btn,
.btn-group-vertical > .btn-group + .btn-group {
  margin-top: -1px;
  margin-left: 0;
}
.btn-group-vertical > .btn:not(:first-child):not(:last-child) {
  border-radius: 0;
}
.btn-group-vertical > .btn:first-child:not(:last-child) {
  border-bottom-right-radius: 0;
  border-bottom-left-radius: 0;
}
.btn-group-vertical > .btn:last-child:not(:first-child) {
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}
.btn-group-vertical > .btn-group:not(:first-child):not(:last-child) > .btn {
  border-radius: 0;
}
.btn-group-vertical > .btn-group:first-child:not(:last-child) > .btn:last-child,
.btn-group-vertical > .btn-group:first-child:not(:last-child) > .dropdown-toggle {
  border-bottom-right-radius: 0;
  border-bottom-left-radius: 0;
}
.btn-group-vertical > .btn-group:last-child:not(:first-child) > .btn:first-child {
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}
[data-toggle=&quot;buttons&quot;] > .btn input[type=&quot;radio&quot;],
[data-toggle=&quot;buttons&quot;] > .btn input[type=&quot;checkbox&quot;],
[data-toggle=&quot;buttons&quot;] > .btn-group > .btn input[type=&quot;radio&quot;],
[data-toggle=&quot;buttons&quot;] > .btn-group > .btn input[type=&quot;checkbox&quot;] {
  position: absolute;
  clip: rect(0, 0, 0, 0);
  pointer-events: none;
}
.input-group {
  position: relative;
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: stretch;
      align-items: stretch;
  width: 100%;
}
.input-group .form-control {
  position: relative;
  z-index: 2;
  -ms-flex: 1 1 auto;
      flex: 1 1 auto;
  width: 1%;
  margin-bottom: 0;
}
.input-group .form-control:focus, .input-group .form-control:active, .input-group .form-control:hover {
  z-index: 3;
}
.input-group-addon,
.input-group-btn,
.input-group .form-control {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: center;
      align-items: center;
}
.input-group-addon:not(:first-child):not(:last-child),
.input-group-btn:not(:first-child):not(:last-child),
.input-group .form-control:not(:first-child):not(:last-child) {
  border-radius: 0;
}
.input-group-addon,
.input-group-btn {
  white-space: nowrap;
}
.input-group-addon {
  padding: 0.375rem 0.75rem;
  margin-bottom: 0;
  font-size: 1rem;
  font-weight: 400;
  line-height: 1.5;
  color: #495057;
  text-align: center;
  background-color: #e9ecef;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
}
.input-group-addon.form-control-sm,
.input-group-sm > .input-group-addon,
.input-group-sm > .input-group-btn > .input-group-addon.btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  border-radius: 0.2rem;
}
.input-group-addon.form-control-lg,
.input-group-lg > .input-group-addon,
.input-group-lg > .input-group-btn > .input-group-addon.btn {
  padding: 0.5rem 1rem;
  font-size: 1.25rem;
  border-radius: 0.3rem;
}
.input-group-addon input[type=&quot;radio&quot;],
.input-group-addon input[type=&quot;checkbox&quot;] {
  margin-top: 0;
}
.input-group .form-control:not(:last-child),
.input-group-addon:not(:last-child),
.input-group-btn:not(:last-child) > .btn,
.input-group-btn:not(:last-child) > .btn-group > .btn,
.input-group-btn:not(:last-child) > .dropdown-toggle,
.input-group-btn:not(:first-child) > .btn:not(:last-child):not(.dropdown-toggle),
.input-group-btn:not(:first-child) > .btn-group:not(:last-child) > .btn {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}
.input-group-addon:not(:last-child) {
  border-right: 0;
}
.input-group .form-control:not(:first-child),
.input-group-addon:not(:first-child),
.input-group-btn:not(:first-child) > .btn,
.input-group-btn:not(:first-child) > .btn-group > .btn,
.input-group-btn:not(:first-child) > .dropdown-toggle,
.input-group-btn:not(:last-child) > .btn:not(:first-child),
.input-group-btn:not(:last-child) > .btn-group:not(:first-child) > .btn {
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}
.form-control + .input-group-addon:not(:first-child) {
  border-left: 0;
}
.input-group-btn {
  position: relative;
  -ms-flex-align: stretch;
      align-items: stretch;
  font-size: 0;
  white-space: nowrap;
}
.input-group-btn > .btn {
  position: relative;
}
.input-group-btn > .btn + .btn {
  margin-left: -1px;
}
.input-group-btn > .btn:focus, .input-group-btn > .btn:active, .input-group-btn > .btn:hover {
  z-index: 3;
}
.input-group-btn:first-child > .btn + .btn {
  margin-left: 0;
}
.input-group-btn:not(:last-child) > .btn,
.input-group-btn:not(:last-child) > .btn-group {
  margin-right: -1px;
}
.input-group-btn:not(:first-child) > .btn,
.input-group-btn:not(:first-child) > .btn-group {
  z-index: 2;
  margin-left: 0;
}
.input-group-btn:not(:first-child) > .btn:first-child,
.input-group-btn:not(:first-child) > .btn-group:first-child {
  margin-left: -1px;
}
.input-group-btn:not(:first-child) > .btn:focus, .input-group-btn:not(:first-child) > .btn:active, .input-group-btn:not(:first-child) > .btn:hover,
.input-group-btn:not(:first-child) > .btn-group:focus,
.input-group-btn:not(:first-child) > .btn-group:active,
.input-group-btn:not(:first-child) > .btn-group:hover {
  z-index: 3;
}
.custom-control {
  position: relative;
  display: -ms-inline-flexbox;
  display: inline-flex;
  min-height: 1.5rem;
  padding-left: 1.5rem;
  margin-right: 1rem;
}
.custom-control-input {
  position: absolute;
  z-index: -1;
  opacity: 0;
}
.custom-control-input:checked ~ .custom-control-indicator {
  color: #fff;
  background-color: #007bff;
}
.custom-control-input:focus ~ .custom-control-indicator {
  box-shadow: 0 0 0 1px #fff, 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}
.custom-control-input:active ~ .custom-control-indicator {
  color: #fff;
  background-color: #b3d7ff;
}
.custom-control-input:disabled ~ .custom-control-indicator {
  background-color: #e9ecef;
}
.custom-control-input:disabled ~ .custom-control-description {
  color: #868e96;
}
.custom-control-indicator {
  position: absolute;
  top: 0.25rem;
  left: 0;
  display: block;
  width: 1rem;
  height: 1rem;
  pointer-events: none;
  -webkit-user-select: none;
     -moz-user-select: none;
      -ms-user-select: none;
          user-select: none;
  background-color: #ddd;
  background-repeat: no-repeat;
  background-position: center center;
  background-size: 50% 50%;
}
.custom-checkbox .custom-control-indicator {
  border-radius: 0.25rem;
}
.custom-checkbox .custom-control-input:checked ~ .custom-control-indicator {
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 8 8'%3E%3Cpath fill='%23fff' d='M6.564.75l-3.59 3.612-1.538-1.55L0 4.26 2.974 7.25 8 2.193z'/%3E%3C/svg%3E&quot;);
}
.custom-checkbox .custom-control-input:indeterminate ~ .custom-control-indicator {
  background-color: #007bff;
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 4'%3E%3Cpath stroke='%23fff' d='M0 2h4'/%3E%3C/svg%3E&quot;);
}
.custom-radio .custom-control-indicator {
  border-radius: 50%;
}
.custom-radio .custom-control-input:checked ~ .custom-control-indicator {
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='-4 -4 8 8'%3E%3Ccircle r='3' fill='%23fff'/%3E%3C/svg%3E&quot;);
}
.custom-controls-stacked {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
}
.custom-controls-stacked .custom-control {
  margin-bottom: 0.25rem;
}
.custom-controls-stacked .custom-control + .custom-control {
  margin-left: 0;
}
.custom-select {
  display: inline-block;
  max-width: 100%;
  height: calc(2.25rem + 2px);
  padding: 0.375rem 1.75rem 0.375rem 0.75rem;
  line-height: 1.5;
  color: #495057;
  vertical-align: middle;
  background: #fff url(&quot;data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 5'%3E%3Cpath fill='%23333' d='M2 0L0 2h4zm0 5L0 3h4z'/%3E%3C/svg%3E&quot;) no-repeat right 0.75rem center;
  background-size: 8px 10px;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
  -webkit-appearance: none;
     -moz-appearance: none;
          appearance: none;
}
.custom-select:focus {
  border-color: #80bdff;
  outline: none;
}
.custom-select:focus::-ms-value {
  color: #495057;
  background-color: #fff;
}
.custom-select[multiple] {
  height: auto;
  background-image: none;
}
.custom-select:disabled {
  color: #868e96;
  background-color: #e9ecef;
}
.custom-select::-ms-expand {
  opacity: 0;
}
.custom-select-sm {
  height: calc(1.8125rem + 2px);
  padding-top: 0.375rem;
  padding-bottom: 0.375rem;
  font-size: 75%;
}
.custom-file {
  position: relative;
  display: inline-block;
  max-width: 100%;
  height: calc(2.25rem + 2px);
  margin-bottom: 0;
}
.custom-file-input {
  min-width: 14rem;
  max-width: 100%;
  height: calc(2.25rem + 2px);
  margin: 0;
  opacity: 0;
}
.custom-file-input:focus ~ .custom-file-control {
  box-shadow: 0 0 0 0.075rem #fff, 0 0 0 0.2rem #007bff;
}
.custom-file-control {
  position: absolute;
  top: 0;
  right: 0;
  left: 0;
  z-index: 5;
  height: calc(2.25rem + 2px);
  padding: 0.375rem 0.75rem;
  line-height: 1.5;
  color: #495057;
  pointer-events: none;
  -webkit-user-select: none;
     -moz-user-select: none;
      -ms-user-select: none;
          user-select: none;
  background-color: #fff;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
}
.custom-file-control:lang(en):empty::after {
  content: &quot;Choose file...&quot;;
}
.custom-file-control::before {
  position: absolute;
  top: -1px;
  right: -1px;
  bottom: -1px;
  z-index: 6;
  display: block;
  height: calc(2.25rem + 2px);
  padding: 0.375rem 0.75rem;
  line-height: 1.5;
  color: #495057;
  background-color: #e9ecef;
  border: 1px solid #ced4da;
  border-radius: 0 0.25rem 0.25rem 0;
}
.custom-file-control:lang(en)::before {
  content: &quot;Browse&quot;;
}
.nav {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  padding-left: 0;
  margin-bottom: 0;
  list-style: none;
}
.nav-link {
  display: block;
  padding: 0.5rem 1rem;
}
.nav-link:focus, .nav-link:hover {
  text-decoration: none;
}
.nav-link.disabled {
  color: #868e96;
}
.nav-tabs {
  border-bottom: 1px solid #ddd;
}
.nav-tabs .nav-item {
  margin-bottom: -1px;
}
.nav-tabs .nav-link {
  border: 1px solid transparent;
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}
.nav-tabs .nav-link:focus, .nav-tabs .nav-link:hover {
  border-color: #e9ecef #e9ecef #ddd;
}
.nav-tabs .nav-link.disabled {
  color: #868e96;
  background-color: transparent;
  border-color: transparent;
}
.nav-tabs .nav-link.active,
.nav-tabs .nav-item.show .nav-link {
  color: #495057;
  background-color: #fff;
  border-color: #ddd #ddd #fff;
}
.nav-tabs .dropdown-menu {
  margin-top: -1px;
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}
.nav-pills .nav-link {
  border-radius: 0.25rem;
}
.nav-pills .nav-link.active,
.nav-pills .show > .nav-link {
  color: #fff;
  background-color: #007bff;
}
.nav-fill .nav-item {
  -ms-flex: 1 1 auto;
      flex: 1 1 auto;
  text-align: center;
}
.nav-justified .nav-item {
  -ms-flex-preferred-size: 0;
      flex-basis: 0;
  -ms-flex-positive: 1;
      flex-grow: 1;
  text-align: center;
}
.tab-content > .tab-pane {
  display: none;
}
.tab-content > .active {
  display: block;
}
.navbar {
  position: relative;
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  -ms-flex-align: center;
      align-items: center;
  -ms-flex-pack: justify;
      justify-content: space-between;
  padding: 0.5rem 1rem;
}
.navbar > .container,
.navbar > .container-fluid {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  -ms-flex-align: center;
      align-items: center;
  -ms-flex-pack: justify;
      justify-content: space-between;
}
.navbar-brand {
  display: inline-block;
  padding-top: 0.3125rem;
  padding-bottom: 0.3125rem;
  margin-right: 1rem;
  font-size: 1.25rem;
  line-height: inherit;
  white-space: nowrap;
}
.navbar-brand:focus, .navbar-brand:hover {
  text-decoration: none;
}
.navbar-nav {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
  padding-left: 0;
  margin-bottom: 0;
  list-style: none;
}
.navbar-nav .nav-link {
  padding-right: 0;
  padding-left: 0;
}
.navbar-nav .dropdown-menu {
  position: static;
  float: none;
}
.navbar-text {
  display: inline-block;
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}
.navbar-collapse {
  -ms-flex-preferred-size: 100%;
      flex-basis: 100%;
  -ms-flex-positive: 1;
      flex-grow: 1;
  -ms-flex-align: center;
      align-items: center;
}
.navbar-toggler {
  padding: 0.25rem 0.75rem;
  font-size: 1.25rem;
  line-height: 1;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 0.25rem;
}
.navbar-toggler:focus, .navbar-toggler:hover {
  text-decoration: none;
}
.navbar-toggler-icon {
  display: inline-block;
  width: 1.5em;
  height: 1.5em;
  vertical-align: middle;
  content: &quot;&quot;;
  background: no-repeat center center;
  background-size: 100% 100%;
}
@media (max-width: 575px) {
  .navbar-expand-sm > .container,
  .navbar-expand-sm > .container-fluid {
    padding-right: 0;
    padding-left: 0;
  }
}
@media (min-width: 576px) {
  .navbar-expand-sm {
    -ms-flex-flow: row nowrap;
        flex-flow: row nowrap;
    -ms-flex-pack: start;
        justify-content: flex-start;
  }
  .navbar-expand-sm .navbar-nav {
    -ms-flex-direction: row;
        flex-direction: row;
  }
  .navbar-expand-sm .navbar-nav .dropdown-menu {
    position: absolute;
  }
  .navbar-expand-sm .navbar-nav .dropdown-menu-right {
    right: 0;
    left: auto;
  }
  .navbar-expand-sm .navbar-nav .nav-link {
    padding-right: .5rem;
    padding-left: .5rem;
  }
  .navbar-expand-sm > .container,
  .navbar-expand-sm > .container-fluid {
    -ms-flex-wrap: nowrap;
        flex-wrap: nowrap;
  }
  .navbar-expand-sm .navbar-collapse {
    display: -ms-flexbox !important;
    display: flex !important;
    -ms-flex-preferred-size: auto;
        flex-basis: auto;
  }
  .navbar-expand-sm .navbar-toggler {
    display: none;
  }
  .navbar-expand-sm .dropup .dropdown-menu {
    top: auto;
    bottom: 100%;
  }
}
@media (max-width: 767px) {
  .navbar-expand-md > .container,
  .navbar-expand-md > .container-fluid {
    padding-right: 0;
    padding-left: 0;
  }
}
@media (min-width: 768px) {
  .navbar-expand-md {
    -ms-flex-flow: row nowrap;
        flex-flow: row nowrap;
    -ms-flex-pack: start;
        justify-content: flex-start;
  }
  .navbar-expand-md .navbar-nav {
    -ms-flex-direction: row;
        flex-direction: row;
  }
  .navbar-expand-md .navbar-nav .dropdown-menu {
    position: absolute;
  }
  .navbar-expand-md .navbar-nav .dropdown-menu-right {
    right: 0;
    left: auto;
  }
  .navbar-expand-md .navbar-nav .nav-link {
    padding-right: .5rem;
    padding-left: .5rem;
  }
  .navbar-expand-md > .container,
  .navbar-expand-md > .container-fluid {
    -ms-flex-wrap: nowrap;
        flex-wrap: nowrap;
  }
  .navbar-expand-md .navbar-collapse {
    display: -ms-flexbox !important;
    display: flex !important;
    -ms-flex-preferred-size: auto;
        flex-basis: auto;
  }
  .navbar-expand-md .navbar-toggler {
    display: none;
  }
  .navbar-expand-md .dropup .dropdown-menu {
    top: auto;
    bottom: 100%;
  }
}
@media (max-width: 991px) {
  .navbar-expand-lg > .container,
  .navbar-expand-lg > .container-fluid {
    padding-right: 0;
    padding-left: 0;
  }
}
@media (min-width: 992px) {
  .navbar-expand-lg {
    -ms-flex-flow: row nowrap;
        flex-flow: row nowrap;
    -ms-flex-pack: start;
        justify-content: flex-start;
  }
  .navbar-expand-lg .navbar-nav {
    -ms-flex-direction: row;
        flex-direction: row;
  }
  .navbar-expand-lg .navbar-nav .dropdown-menu {
    position: absolute;
  }
  .navbar-expand-lg .navbar-nav .dropdown-menu-right {
    right: 0;
    left: auto;
  }
  .navbar-expand-lg .navbar-nav .nav-link {
    padding-right: .5rem;
    padding-left: .5rem;
  }
  .navbar-expand-lg > .container,
  .navbar-expand-lg > .container-fluid {
    -ms-flex-wrap: nowrap;
        flex-wrap: nowrap;
  }
  .navbar-expand-lg .navbar-collapse {
    display: -ms-flexbox !important;
    display: flex !important;
    -ms-flex-preferred-size: auto;
        flex-basis: auto;
  }
  .navbar-expand-lg .navbar-toggler {
    display: none;
  }
  .navbar-expand-lg .dropup .dropdown-menu {
    top: auto;
    bottom: 100%;
  }
}
@media (max-width: 1199px) {
  .navbar-expand-xl > .container,
  .navbar-expand-xl > .container-fluid {
    padding-right: 0;
    padding-left: 0;
  }
}
@media (min-width: 1200px) {
  .navbar-expand-xl {
    -ms-flex-flow: row nowrap;
        flex-flow: row nowrap;
    -ms-flex-pack: start;
        justify-content: flex-start;
  }
  .navbar-expand-xl .navbar-nav {
    -ms-flex-direction: row;
        flex-direction: row;
  }
  .navbar-expand-xl .navbar-nav .dropdown-menu {
    position: absolute;
  }
  .navbar-expand-xl .navbar-nav .dropdown-menu-right {
    right: 0;
    left: auto;
  }
  .navbar-expand-xl .navbar-nav .nav-link {
    padding-right: .5rem;
    padding-left: .5rem;
  }
  .navbar-expand-xl > .container,
  .navbar-expand-xl > .container-fluid {
    -ms-flex-wrap: nowrap;
        flex-wrap: nowrap;
  }
  .navbar-expand-xl .navbar-collapse {
    display: -ms-flexbox !important;
    display: flex !important;
    -ms-flex-preferred-size: auto;
        flex-basis: auto;
  }
  .navbar-expand-xl .navbar-toggler {
    display: none;
  }
  .navbar-expand-xl .dropup .dropdown-menu {
    top: auto;
    bottom: 100%;
  }
}
.navbar-expand {
  -ms-flex-flow: row nowrap;
      flex-flow: row nowrap;
  -ms-flex-pack: start;
      justify-content: flex-start;
}
.navbar-expand > .container,
.navbar-expand > .container-fluid {
  padding-right: 0;
  padding-left: 0;
}
.navbar-expand .navbar-nav {
  -ms-flex-direction: row;
      flex-direction: row;
}
.navbar-expand .navbar-nav .dropdown-menu {
  position: absolute;
}
.navbar-expand .navbar-nav .dropdown-menu-right {
  right: 0;
  left: auto;
}
.navbar-expand .navbar-nav .nav-link {
  padding-right: .5rem;
  padding-left: .5rem;
}
.navbar-expand > .container,
.navbar-expand > .container-fluid {
  -ms-flex-wrap: nowrap;
      flex-wrap: nowrap;
}
.navbar-expand .navbar-collapse {
  display: -ms-flexbox !important;
  display: flex !important;
  -ms-flex-preferred-size: auto;
      flex-basis: auto;
}
.navbar-expand .navbar-toggler {
  display: none;
}
.navbar-expand .dropup .dropdown-menu {
  top: auto;
  bottom: 100%;
}
.navbar-light .navbar-brand {
  color: rgba(0, 0, 0, 0.9);
}
.navbar-light .navbar-brand:focus, .navbar-light .navbar-brand:hover {
  color: rgba(0, 0, 0, 0.9);
}
.navbar-light .navbar-nav .nav-link {
  color: rgba(0, 0, 0, 0.5);
}
.navbar-light .navbar-nav .nav-link:focus, .navbar-light .navbar-nav .nav-link:hover {
  color: rgba(0, 0, 0, 0.7);
}
.navbar-light .navbar-nav .nav-link.disabled {
  color: rgba(0, 0, 0, 0.3);
}
.navbar-light .navbar-nav .show > .nav-link,
.navbar-light .navbar-nav .active > .nav-link,
.navbar-light .navbar-nav .nav-link.show,
.navbar-light .navbar-nav .nav-link.active {
  color: rgba(0, 0, 0, 0.9);
}
.navbar-light .navbar-toggler {
  color: rgba(0, 0, 0, 0.5);
  border-color: rgba(0, 0, 0, 0.1);
}
.navbar-light .navbar-toggler-icon {
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg viewBox='0 0 30 30' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath stroke='rgba(0, 0, 0, 0.5)' stroke-width='2' stroke-linecap='round' stroke-miterlimit='10' d='M4 7h22M4 15h22M4 23h22'/%3E%3C/svg%3E&quot;);
}
.navbar-light .navbar-text {
  color: rgba(0, 0, 0, 0.5);
}
.navbar-light .navbar-text a {
  color: rgba(0, 0, 0, 0.9);
}
.navbar-light .navbar-text a:focus, .navbar-light .navbar-text a:hover {
  color: rgba(0, 0, 0, 0.9);
}
.navbar-dark .navbar-brand {
  color: #fff;
}
.navbar-dark .navbar-brand:focus, .navbar-dark .navbar-brand:hover {
  color: #fff;
}
.navbar-dark .navbar-nav .nav-link {
  color: rgba(255, 255, 255, 0.5);
}
.navbar-dark .navbar-nav .nav-link:focus, .navbar-dark .navbar-nav .nav-link:hover {
  color: rgba(255, 255, 255, 0.75);
}
.navbar-dark .navbar-nav .nav-link.disabled {
  color: rgba(255, 255, 255, 0.25);
}
.navbar-dark .navbar-nav .show > .nav-link,
.navbar-dark .navbar-nav .active > .nav-link,
.navbar-dark .navbar-nav .nav-link.show,
.navbar-dark .navbar-nav .nav-link.active {
  color: #fff;
}
.navbar-dark .navbar-toggler {
  color: rgba(255, 255, 255, 0.5);
  border-color: rgba(255, 255, 255, 0.1);
}
.navbar-dark .navbar-toggler-icon {
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg viewBox='0 0 30 30' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath stroke='rgba(255, 255, 255, 0.5)' stroke-width='2' stroke-linecap='round' stroke-miterlimit='10' d='M4 7h22M4 15h22M4 23h22'/%3E%3C/svg%3E&quot;);
}
.navbar-dark .navbar-text {
  color: rgba(255, 255, 255, 0.5);
}
.navbar-dark .navbar-text a {
  color: #fff;
}
.navbar-dark .navbar-text a:focus, .navbar-dark .navbar-text a:hover {
  color: #fff;
}
.card {
  position: relative;
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
  min-width: 0;
  word-wrap: break-word;
  background-color: #fff;
  background-clip: border-box;
  border: 1px solid rgba(0, 0, 0, 0.125);
  border-radius: 0.25rem;
}
.card > hr {
  margin-right: 0;
  margin-left: 0;
}
.card > .list-group:first-child .list-group-item:first-child {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}
.card > .list-group:last-child .list-group-item:last-child {
  border-bottom-right-radius: 0.25rem;
  border-bottom-left-radius: 0.25rem;
}
.card-body {
  -ms-flex: 1 1 auto;
      flex: 1 1 auto;
  padding: 1.25rem;
}
.card-title {
  margin-bottom: 0.75rem;
}
.card-subtitle {
  margin-top: -0.375rem;
  margin-bottom: 0;
}
.card-text:last-child {
  margin-bottom: 0;
}
.card-link:hover {
  text-decoration: none;
}
.card-link + .card-link {
  margin-left: 1.25rem;
}
.card-header {
  padding: 0.75rem 1.25rem;
  margin-bottom: 0;
  background-color: rgba(0, 0, 0, 0.03);
  border-bottom: 1px solid rgba(0, 0, 0, 0.125);
}
.card-header:first-child {
  border-radius: calc(0.25rem - 1px) calc(0.25rem - 1px) 0 0;
}
.card-header + .list-group .list-group-item:first-child {
  border-top: 0;
}
.card-footer {
  padding: 0.75rem 1.25rem;
  background-color: rgba(0, 0, 0, 0.03);
  border-top: 1px solid rgba(0, 0, 0, 0.125);
}
.card-footer:last-child {
  border-radius: 0 0 calc(0.25rem - 1px) calc(0.25rem - 1px);
}
.card-header-tabs {
  margin-right: -0.625rem;
  margin-bottom: -0.75rem;
  margin-left: -0.625rem;
  border-bottom: 0;
}
.card-header-pills {
  margin-right: -0.625rem;
  margin-left: -0.625rem;
}
.card-img-overlay {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  padding: 1.25rem;
}
.card-img {
  width: 100%;
  border-radius: calc(0.25rem - 1px);
}
.card-img-top {
  width: 100%;
  border-top-left-radius: calc(0.25rem - 1px);
  border-top-right-radius: calc(0.25rem - 1px);
}
.card-img-bottom {
  width: 100%;
  border-bottom-right-radius: calc(0.25rem - 1px);
  border-bottom-left-radius: calc(0.25rem - 1px);
}
.card-deck {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
}
.card-deck .card {
  margin-bottom: 15px;
}
@media (min-width: 576px) {
  .card-deck {
    -ms-flex-flow: row wrap;
        flex-flow: row wrap;
    margin-right: -15px;
    margin-left: -15px;
  }
  .card-deck .card {
    display: -ms-flexbox;
    display: flex;
    -ms-flex: 1 0 0%;
        flex: 1 0 0%;
    -ms-flex-direction: column;
        flex-direction: column;
    margin-right: 15px;
    margin-bottom: 0;
    margin-left: 15px;
  }
}
.card-group {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
}
.card-group .card {
  margin-bottom: 15px;
}
@media (min-width: 576px) {
  .card-group {
    -ms-flex-flow: row wrap;
        flex-flow: row wrap;
  }
  .card-group .card {
    -ms-flex: 1 0 0%;
        flex: 1 0 0%;
    margin-bottom: 0;
  }
  .card-group .card + .card {
    margin-left: 0;
    border-left: 0;
  }
  .card-group .card:first-child {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }
  .card-group .card:first-child .card-img-top {
    border-top-right-radius: 0;
  }
  .card-group .card:first-child .card-img-bottom {
    border-bottom-right-radius: 0;
  }
  .card-group .card:last-child {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
  .card-group .card:last-child .card-img-top {
    border-top-left-radius: 0;
  }
  .card-group .card:last-child .card-img-bottom {
    border-bottom-left-radius: 0;
  }
  .card-group .card:only-child {
    border-radius: 0.25rem;
  }
  .card-group .card:only-child .card-img-top {
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
  }
  .card-group .card:only-child .card-img-bottom {
    border-bottom-right-radius: 0.25rem;
    border-bottom-left-radius: 0.25rem;
  }
  .card-group .card:not(:first-child):not(:last-child):not(:only-child) {
    border-radius: 0;
  }
  .card-group .card:not(:first-child):not(:last-child):not(:only-child) .card-img-top,
  .card-group .card:not(:first-child):not(:last-child):not(:only-child) .card-img-bottom {
    border-radius: 0;
  }
}
.card-columns .card {
  margin-bottom: 0.75rem;
}
@media (min-width: 576px) {
  .card-columns {
    -webkit-column-count: 3;
            column-count: 3;
    -webkit-column-gap: 1.25rem;
            column-gap: 1.25rem;
  }
  .card-columns .card {
    display: inline-block;
    width: 100%;
  }
}
.breadcrumb {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-wrap: wrap;
      flex-wrap: wrap;
  padding: 0.75rem 1rem;
  margin-bottom: 1rem;
  list-style: none;
  background-color: #e9ecef;
  border-radius: 0.25rem;
}
.breadcrumb-item + .breadcrumb-item::before {
  display: inline-block;
  padding-right: 0.5rem;
  padding-left: 0.5rem;
  color: #868e96;
  content: &quot;/&quot;;
}
.breadcrumb-item + .breadcrumb-item:hover::before {
  text-decoration: underline;
}
.breadcrumb-item + .breadcrumb-item:hover::before {
  text-decoration: none;
}
.breadcrumb-item.active {
  color: #868e96;
}
.pagination {
  display: -ms-flexbox;
  display: flex;
  padding-left: 0;
  list-style: none;
  border-radius: 0.25rem;
}
.page-item:first-child .page-link {
  margin-left: 0;
  border-top-left-radius: 0.25rem;
  border-bottom-left-radius: 0.25rem;
}
.page-item:last-child .page-link {
  border-top-right-radius: 0.25rem;
  border-bottom-right-radius: 0.25rem;
}
.page-item.active .page-link {
  z-index: 2;
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
}
.page-item.disabled .page-link {
  color: #868e96;
  pointer-events: none;
  background-color: #fff;
  border-color: #ddd;
}
.page-link {
  position: relative;
  display: block;
  padding: 0.5rem 0.75rem;
  margin-left: -1px;
  line-height: 1.25;
  color: #007bff;
  background-color: #fff;
  border: 1px solid #ddd;
}
.page-link:focus, .page-link:hover {
  color: #0056b3;
  text-decoration: none;
  background-color: #e9ecef;
  border-color: #ddd;
}
.pagination-lg .page-link {
  padding: 0.75rem 1.5rem;
  font-size: 1.25rem;
  line-height: 1.5;
}
.pagination-lg .page-item:first-child .page-link {
  border-top-left-radius: 0.3rem;
  border-bottom-left-radius: 0.3rem;
}
.pagination-lg .page-item:last-child .page-link {
  border-top-right-radius: 0.3rem;
  border-bottom-right-radius: 0.3rem;
}
.pagination-sm .page-link {
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  line-height: 1.5;
}
.pagination-sm .page-item:first-child .page-link {
  border-top-left-radius: 0.2rem;
  border-bottom-left-radius: 0.2rem;
}
.pagination-sm .page-item:last-child .page-link {
  border-top-right-radius: 0.2rem;
  border-bottom-right-radius: 0.2rem;
}
.badge {
  display: inline-block;
  padding: 0.25em 0.4em;
  font-size: 75%;
  font-weight: 700;
  line-height: 1;
  text-align: center;
  white-space: nowrap;
  vertical-align: baseline;
  border-radius: 0.25rem;
}
.badge:empty {
  display: none;
}
.btn .badge {
  position: relative;
  top: -1px;
}
.badge-pill {
  padding-right: 0.6em;
  padding-left: 0.6em;
  border-radius: 10rem;
}
.badge-primary {
  color: #fff;
  background-color: #007bff;
}
.badge-primary[href]:focus, .badge-primary[href]:hover {
  color: #fff;
  text-decoration: none;
  background-color: #0062cc;
}
.badge-secondary {
  color: #fff;
  background-color: #868e96;
}
.badge-secondary[href]:focus, .badge-secondary[href]:hover {
  color: #fff;
  text-decoration: none;
  background-color: #6c757d;
}
.badge-success {
  color: #fff;
  background-color: #28a745;
}
.badge-success[href]:focus, .badge-success[href]:hover {
  color: #fff;
  text-decoration: none;
  background-color: #1e7e34;
}
.badge-info {
  color: #fff;
  background-color: #17a2b8;
}
.badge-info[href]:focus, .badge-info[href]:hover {
  color: #fff;
  text-decoration: none;
  background-color: #117a8b;
}
.badge-warning {
  color: #111;
  background-color: #ffc107;
}
.badge-warning[href]:focus, .badge-warning[href]:hover {
  color: #111;
  text-decoration: none;
  background-color: #d39e00;
}
.badge-danger {
  color: #fff;
  background-color: #dc3545;
}
.badge-danger[href]:focus, .badge-danger[href]:hover {
  color: #fff;
  text-decoration: none;
  background-color: #bd2130;
}
.badge-light {
  color: #111;
  background-color: #f8f9fa;
}
.badge-light[href]:focus, .badge-light[href]:hover {
  color: #111;
  text-decoration: none;
  background-color: #dae0e5;
}
.badge-dark {
  color: #fff;
  background-color: #343a40;
}
.badge-dark[href]:focus, .badge-dark[href]:hover {
  color: #fff;
  text-decoration: none;
  background-color: #1d2124;
}
.jumbotron {
  padding: 2rem 1rem;
  margin-bottom: 2rem;
  background-color: #e9ecef;
  border-radius: 0.3rem;
}
@media (min-width: 576px) {
  .jumbotron {
    padding: 4rem 2rem;
  }
}
.jumbotron-fluid {
  padding-right: 0;
  padding-left: 0;
  border-radius: 0;
}
.alert {
  position: relative;
  padding: 0.75rem 1.25rem;
  margin-bottom: 1rem;
  border: 1px solid transparent;
  border-radius: 0.25rem;
}
.alert-heading {
  color: inherit;
}
.alert-link {
  font-weight: 700;
}
.alert-dismissible .close {
  position: absolute;
  top: 0;
  right: 0;
  padding: 0.75rem 1.25rem;
  color: inherit;
}
.alert-primary {
  color: #004085;
  background-color: #cce5ff;
  border-color: #b8daff;
}
.alert-primary hr {
  border-top-color: #9fcdff;
}
.alert-primary .alert-link {
  color: #002752;
}
.alert-secondary {
  color: #464a4e;
  background-color: #e7e8ea;
  border-color: #dddfe2;
}
.alert-secondary hr {
  border-top-color: #cfd2d6;
}
.alert-secondary .alert-link {
  color: #2e3133;
}
.alert-success {
  color: #155724;
  background-color: #d4edda;
  border-color: #c3e6cb;
}
.alert-success hr {
  border-top-color: #b1dfbb;
}
.alert-success .alert-link {
  color: #0b2e13;
}
.alert-info {
  color: #0c5460;
  background-color: #d1ecf1;
  border-color: #bee5eb;
}
.alert-info hr {
  border-top-color: #abdde5;
}
.alert-info .alert-link {
  color: #062c33;
}
.alert-warning {
  color: #856404;
  background-color: #fff3cd;
  border-color: #ffeeba;
}
.alert-warning hr {
  border-top-color: #ffe8a1;
}
.alert-warning .alert-link {
  color: #533f03;
}
.alert-danger {
  color: #721c24;
  background-color: #f8d7da;
  border-color: #f5c6cb;
}
.alert-danger hr {
  border-top-color: #f1b0b7;
}
.alert-danger .alert-link {
  color: #491217;
}
.alert-light {
  color: #818182;
  background-color: #fefefe;
  border-color: #fdfdfe;
}
.alert-light hr {
  border-top-color: #ececf6;
}
.alert-light .alert-link {
  color: #686868;
}
.alert-dark {
  color: #1b1e21;
  background-color: #d6d8d9;
  border-color: #c6c8ca;
}
.alert-dark hr {
  border-top-color: #b9bbbe;
}
.alert-dark .alert-link {
  color: #040505;
}
@-webkit-keyframes progress-bar-stripes {
  from {
    background-position: 1rem 0;
  }
  to {
    background-position: 0 0;
  }
}
@keyframes progress-bar-stripes {
  from {
    background-position: 1rem 0;
  }
  to {
    background-position: 0 0;
  }
}
.progress {
  display: -ms-flexbox;
  display: flex;
  height: 1rem;
  overflow: hidden;
  font-size: 0.75rem;
  background-color: #e9ecef;
  border-radius: 0.25rem;
}
.progress-bar {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: center;
      align-items: center;
  -ms-flex-pack: center;
      justify-content: center;
  color: #fff;
  background-color: #007bff;
}
.progress-bar-striped {
  background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, 0.15) 50%, rgba(255, 255, 255, 0.15) 75%, transparent 75%, transparent);
  background-size: 1rem 1rem;
}
.progress-bar-animated {
  -webkit-animation: progress-bar-stripes 1s linear infinite;
          animation: progress-bar-stripes 1s linear infinite;
}
.media {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: start;
      align-items: flex-start;
}
.media-body {
  -ms-flex: 1;
      flex: 1;
}
.list-group {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
  padding-left: 0;
  margin-bottom: 0;
}
.list-group-item-action {
  width: 100%;
  color: #495057;
  text-align: inherit;
}
.list-group-item-action:focus, .list-group-item-action:hover {
  color: #495057;
  text-decoration: none;
  background-color: #f8f9fa;
}
.list-group-item-action:active {
  color: #212529;
  background-color: #e9ecef;
}
.list-group-item {
  position: relative;
  display: block;
  padding: 0.75rem 1.25rem;
  margin-bottom: -1px;
  background-color: #fff;
  border: 1px solid rgba(0, 0, 0, 0.125);
}
.list-group-item:first-child {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}
.list-group-item:last-child {
  margin-bottom: 0;
  border-bottom-right-radius: 0.25rem;
  border-bottom-left-radius: 0.25rem;
}
.list-group-item:focus, .list-group-item:hover {
  text-decoration: none;
}
.list-group-item.disabled, .list-group-item:disabled {
  color: #868e96;
  background-color: #fff;
}
.list-group-item.active {
  z-index: 2;
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
}
.list-group-flush .list-group-item {
  border-right: 0;
  border-left: 0;
  border-radius: 0;
}
.list-group-flush:first-child .list-group-item:first-child {
  border-top: 0;
}
.list-group-flush:last-child .list-group-item:last-child {
  border-bottom: 0;
}
.list-group-item-primary {
  color: #004085;
  background-color: #b8daff;
}
a.list-group-item-primary,
button.list-group-item-primary {
  color: #004085;
}
a.list-group-item-primary:focus, a.list-group-item-primary:hover,
button.list-group-item-primary:focus,
button.list-group-item-primary:hover {
  color: #004085;
  background-color: #9fcdff;
}
a.list-group-item-primary.active,
button.list-group-item-primary.active {
  color: #fff;
  background-color: #004085;
  border-color: #004085;
}
.list-group-item-secondary {
  color: #464a4e;
  background-color: #dddfe2;
}
a.list-group-item-secondary,
button.list-group-item-secondary {
  color: #464a4e;
}
a.list-group-item-secondary:focus, a.list-group-item-secondary:hover,
button.list-group-item-secondary:focus,
button.list-group-item-secondary:hover {
  color: #464a4e;
  background-color: #cfd2d6;
}
a.list-group-item-secondary.active,
button.list-group-item-secondary.active {
  color: #fff;
  background-color: #464a4e;
  border-color: #464a4e;
}
.list-group-item-success {
  color: #155724;
  background-color: #c3e6cb;
}
a.list-group-item-success,
button.list-group-item-success {
  color: #155724;
}
a.list-group-item-success:focus, a.list-group-item-success:hover,
button.list-group-item-success:focus,
button.list-group-item-success:hover {
  color: #155724;
  background-color: #b1dfbb;
}
a.list-group-item-success.active,
button.list-group-item-success.active {
  color: #fff;
  background-color: #155724;
  border-color: #155724;
}
.list-group-item-info {
  color: #0c5460;
  background-color: #bee5eb;
}
a.list-group-item-info,
button.list-group-item-info {
  color: #0c5460;
}
a.list-group-item-info:focus, a.list-group-item-info:hover,
button.list-group-item-info:focus,
button.list-group-item-info:hover {
  color: #0c5460;
  background-color: #abdde5;
}
a.list-group-item-info.active,
button.list-group-item-info.active {
  color: #fff;
  background-color: #0c5460;
  border-color: #0c5460;
}
.list-group-item-warning {
  color: #856404;
  background-color: #ffeeba;
}
a.list-group-item-warning,
button.list-group-item-warning {
  color: #856404;
}
a.list-group-item-warning:focus, a.list-group-item-warning:hover,
button.list-group-item-warning:focus,
button.list-group-item-warning:hover {
  color: #856404;
  background-color: #ffe8a1;
}
a.list-group-item-warning.active,
button.list-group-item-warning.active {
  color: #fff;
  background-color: #856404;
  border-color: #856404;
}
.list-group-item-danger {
  color: #721c24;
  background-color: #f5c6cb;
}
a.list-group-item-danger,
button.list-group-item-danger {
  color: #721c24;
}
a.list-group-item-danger:focus, a.list-group-item-danger:hover,
button.list-group-item-danger:focus,
button.list-group-item-danger:hover {
  color: #721c24;
  background-color: #f1b0b7;
}
a.list-group-item-danger.active,
button.list-group-item-danger.active {
  color: #fff;
  background-color: #721c24;
  border-color: #721c24;
}
.list-group-item-light {
  color: #818182;
  background-color: #fdfdfe;
}
a.list-group-item-light,
button.list-group-item-light {
  color: #818182;
}
a.list-group-item-light:focus, a.list-group-item-light:hover,
button.list-group-item-light:focus,
button.list-group-item-light:hover {
  color: #818182;
  background-color: #ececf6;
}
a.list-group-item-light.active,
button.list-group-item-light.active {
  color: #fff;
  background-color: #818182;
  border-color: #818182;
}
.list-group-item-dark {
  color: #1b1e21;
  background-color: #c6c8ca;
}
a.list-group-item-dark,
button.list-group-item-dark {
  color: #1b1e21;
}
a.list-group-item-dark:focus, a.list-group-item-dark:hover,
button.list-group-item-dark:focus,
button.list-group-item-dark:hover {
  color: #1b1e21;
  background-color: #b9bbbe;
}
a.list-group-item-dark.active,
button.list-group-item-dark.active {
  color: #fff;
  background-color: #1b1e21;
  border-color: #1b1e21;
}
.close {
  float: right;
  font-size: 1.5rem;
  font-weight: 700;
  line-height: 1;
  color: #000;
  text-shadow: 0 1px 0 #fff;
  opacity: .5;
}
.close:focus, .close:hover {
  color: #000;
  text-decoration: none;
  opacity: .75;
}
button.close {
  padding: 0;
  background: transparent;
  border: 0;
  -webkit-appearance: none;
}
.modal-open {
  overflow: hidden;
}
.modal {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 1050;
  display: none;
  overflow: hidden;
  outline: 0;
}
.modal.fade .modal-dialog {
  transition: -webkit-transform 0.3s ease-out;
  transition: transform 0.3s ease-out;
  transition: transform 0.3s ease-out, -webkit-transform 0.3s ease-out;
  -webkit-transform: translate(0, -25%);
          transform: translate(0, -25%);
}
.modal.show .modal-dialog {
  -webkit-transform: translate(0, 0);
          transform: translate(0, 0);
}
.modal-open .modal {
  overflow-x: hidden;
  overflow-y: auto;
}
.modal-dialog {
  position: relative;
  width: auto;
  margin: 10px;
  pointer-events: none;
}
.modal-content {
  position: relative;
  display: -ms-flexbox;
  display: flex;
  -ms-flex-direction: column;
      flex-direction: column;
  pointer-events: auto;
  background-color: #fff;
  background-clip: padding-box;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 0.3rem;
  outline: 0;
}
.modal-backdrop {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 1040;
  background-color: #000;
}
.modal-backdrop.fade {
  opacity: 0;
}
.modal-backdrop.show {
  opacity: 0.5;
}
.modal-header {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: start;
      align-items: flex-start;
  -ms-flex-pack: justify;
      justify-content: space-between;
  padding: 15px;
  border-bottom: 1px solid #e9ecef;
  border-top-left-radius: 0.3rem;
  border-top-right-radius: 0.3rem;
}
.modal-header .close {
  padding: 15px;
  margin: -15px -15px -15px auto;
}
.modal-title {
  margin-bottom: 0;
  line-height: 1.5;
}
.modal-body {
  position: relative;
  -ms-flex: 1 1 auto;
      flex: 1 1 auto;
  padding: 15px;
}
.modal-footer {
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: center;
      align-items: center;
  -ms-flex-pack: end;
      justify-content: flex-end;
  padding: 15px;
  border-top: 1px solid #e9ecef;
}
.modal-footer > :not(:first-child) {
  margin-left: .25rem;
}
.modal-footer > :not(:last-child) {
  margin-right: .25rem;
}
.modal-scrollbar-measure {
  position: absolute;
  top: -9999px;
  width: 50px;
  height: 50px;
  overflow: scroll;
}
@media (min-width: 576px) {
  .modal-dialog {
    max-width: 500px;
    margin: 30px auto;
  }
  .modal-sm {
    max-width: 300px;
  }
}
@media (min-width: 992px) {
  .modal-lg {
    max-width: 800px;
  }
}
.tooltip {
  position: absolute;
  z-index: 1070;
  display: block;
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Helvetica Neue&quot;, Arial, sans-serif, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Symbol&quot;;
  font-style: normal;
  font-weight: 400;
  line-height: 1.5;
  text-align: left;
  text-align: start;
  text-decoration: none;
  text-shadow: none;
  text-transform: none;
  letter-spacing: normal;
  word-break: normal;
  word-spacing: normal;
  white-space: normal;
  line-break: auto;
  font-size: 0.875rem;
  word-wrap: break-word;
  opacity: 0;
}
.tooltip.show {
  opacity: 0.9;
}
.tooltip .arrow {
  position: absolute;
  display: block;
  width: 5px;
  height: 5px;
}
.tooltip .arrow::before {
  position: absolute;
  border-color: transparent;
  border-style: solid;
}
.tooltip.bs-tooltip-top, .tooltip.bs-tooltip-auto[x-placement^=&quot;top&quot;] {
  padding: 5px 0;
}
.tooltip.bs-tooltip-top .arrow, .tooltip.bs-tooltip-auto[x-placement^=&quot;top&quot;] .arrow {
  bottom: 0;
}
.tooltip.bs-tooltip-top .arrow::before, .tooltip.bs-tooltip-auto[x-placement^=&quot;top&quot;] .arrow::before {
  margin-left: -3px;
  content: &quot;&quot;;
  border-width: 5px 5px 0;
  border-top-color: #000;
}
.tooltip.bs-tooltip-right, .tooltip.bs-tooltip-auto[x-placement^=&quot;right&quot;] {
  padding: 0 5px;
}
.tooltip.bs-tooltip-right .arrow, .tooltip.bs-tooltip-auto[x-placement^=&quot;right&quot;] .arrow {
  left: 0;
}
.tooltip.bs-tooltip-right .arrow::before, .tooltip.bs-tooltip-auto[x-placement^=&quot;right&quot;] .arrow::before {
  margin-top: -3px;
  content: &quot;&quot;;
  border-width: 5px 5px 5px 0;
  border-right-color: #000;
}
.tooltip.bs-tooltip-bottom, .tooltip.bs-tooltip-auto[x-placement^=&quot;bottom&quot;] {
  padding: 5px 0;
}
.tooltip.bs-tooltip-bottom .arrow, .tooltip.bs-tooltip-auto[x-placement^=&quot;bottom&quot;] .arrow {
  top: 0;
}
.tooltip.bs-tooltip-bottom .arrow::before, .tooltip.bs-tooltip-auto[x-placement^=&quot;bottom&quot;] .arrow::before {
  margin-left: -3px;
  content: &quot;&quot;;
  border-width: 0 5px 5px;
  border-bottom-color: #000;
}
.tooltip.bs-tooltip-left, .tooltip.bs-tooltip-auto[x-placement^=&quot;left&quot;] {
  padding: 0 5px;
}
.tooltip.bs-tooltip-left .arrow, .tooltip.bs-tooltip-auto[x-placement^=&quot;left&quot;] .arrow {
  right: 0;
}
.tooltip.bs-tooltip-left .arrow::before, .tooltip.bs-tooltip-auto[x-placement^=&quot;left&quot;] .arrow::before {
  right: 0;
  margin-top: -3px;
  content: &quot;&quot;;
  border-width: 5px 0 5px 5px;
  border-left-color: #000;
}
.tooltip-inner {
  max-width: 200px;
  padding: 3px 8px;
  color: #fff;
  text-align: center;
  background-color: #000;
  border-radius: 0.25rem;
}
.popover {
  position: absolute;
  top: 0;
  left: 0;
  z-index: 1060;
  display: block;
  max-width: 276px;
  font-family: -apple-system, BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Helvetica Neue&quot;, Arial, sans-serif, &quot;Apple Color Emoji&quot;, &quot;Segoe UI Emoji&quot;, &quot;Segoe UI Symbol&quot;;
  font-style: normal;
  font-weight: 400;
  line-height: 1.5;
  text-align: left;
  text-align: start;
  text-decoration: none;
  text-shadow: none;
  text-transform: none;
  letter-spacing: normal;
  word-break: normal;
  word-spacing: normal;
  white-space: normal;
  line-break: auto;
  font-size: 0.875rem;
  word-wrap: break-word;
  background-color: #fff;
  background-clip: padding-box;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 0.3rem;
}
.popover .arrow {
  position: absolute;
  display: block;
  width: 0.8rem;
  height: 0.4rem;
}
.popover .arrow::before,
.popover .arrow::after {
  position: absolute;
  display: block;
  border-color: transparent;
  border-style: solid;
}
.popover .arrow::before {
  content: &quot;&quot;;
  border-width: 0.8rem;
}
.popover .arrow::after {
  content: &quot;&quot;;
  border-width: 0.8rem;
}
.popover.bs-popover-top, .popover.bs-popover-auto[x-placement^=&quot;top&quot;] {
  margin-bottom: 0.8rem;
}
.popover.bs-popover-top .arrow, .popover.bs-popover-auto[x-placement^=&quot;top&quot;] .arrow {
  bottom: 0;
}
.popover.bs-popover-top .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;top&quot;] .arrow::before,
.popover.bs-popover-top .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;top&quot;] .arrow::after {
  border-bottom-width: 0;
}
.popover.bs-popover-top .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;top&quot;] .arrow::before {
  bottom: -0.8rem;
  margin-left: -0.8rem;
  border-top-color: rgba(0, 0, 0, 0.25);
}
.popover.bs-popover-top .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;top&quot;] .arrow::after {
  bottom: calc((0.8rem - 1px) * -1);
  margin-left: -0.8rem;
  border-top-color: #fff;
}
.popover.bs-popover-right, .popover.bs-popover-auto[x-placement^=&quot;right&quot;] {
  margin-left: 0.8rem;
}
.popover.bs-popover-right .arrow, .popover.bs-popover-auto[x-placement^=&quot;right&quot;] .arrow {
  left: 0;
}
.popover.bs-popover-right .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;right&quot;] .arrow::before,
.popover.bs-popover-right .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;right&quot;] .arrow::after {
  margin-top: -0.8rem;
  border-left-width: 0;
}
.popover.bs-popover-right .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;right&quot;] .arrow::before {
  left: -0.8rem;
  border-right-color: rgba(0, 0, 0, 0.25);
}
.popover.bs-popover-right .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;right&quot;] .arrow::after {
  left: calc((0.8rem - 1px) * -1);
  border-right-color: #fff;
}
.popover.bs-popover-bottom, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] {
  margin-top: 0.8rem;
}
.popover.bs-popover-bottom .arrow, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] .arrow {
  top: 0;
}
.popover.bs-popover-bottom .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] .arrow::before,
.popover.bs-popover-bottom .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] .arrow::after {
  margin-left: -0.8rem;
  border-top-width: 0;
}
.popover.bs-popover-bottom .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] .arrow::before {
  top: -0.8rem;
  border-bottom-color: rgba(0, 0, 0, 0.25);
}
.popover.bs-popover-bottom .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] .arrow::after {
  top: calc((0.8rem - 1px) * -1);
  border-bottom-color: #fff;
}
.popover.bs-popover-bottom .popover-header::before, .popover.bs-popover-auto[x-placement^=&quot;bottom&quot;] .popover-header::before {
  position: absolute;
  top: 0;
  left: 50%;
  display: block;
  width: 20px;
  margin-left: -10px;
  content: &quot;&quot;;
  border-bottom: 1px solid #f7f7f7;
}
.popover.bs-popover-left, .popover.bs-popover-auto[x-placement^=&quot;left&quot;] {
  margin-right: 0.8rem;
}
.popover.bs-popover-left .arrow, .popover.bs-popover-auto[x-placement^=&quot;left&quot;] .arrow {
  right: 0;
}
.popover.bs-popover-left .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;left&quot;] .arrow::before,
.popover.bs-popover-left .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;left&quot;] .arrow::after {
  margin-top: -0.8rem;
  border-right-width: 0;
}
.popover.bs-popover-left .arrow::before, .popover.bs-popover-auto[x-placement^=&quot;left&quot;] .arrow::before {
  right: -0.8rem;
  border-left-color: rgba(0, 0, 0, 0.25);
}
.popover.bs-popover-left .arrow::after, .popover.bs-popover-auto[x-placement^=&quot;left&quot;] .arrow::after {
  right: calc((0.8rem - 1px) * -1);
  border-left-color: #fff;
}
.popover-header {
  padding: 0.5rem 0.75rem;
  margin-bottom: 0;
  font-size: 1rem;
  color: inherit;
  background-color: #f7f7f7;
  border-bottom: 1px solid #ebebeb;
  border-top-left-radius: calc(0.3rem - 1px);
  border-top-right-radius: calc(0.3rem - 1px);
}
.popover-header:empty {
  display: none;
}
.popover-body {
  padding: 0.5rem 0.75rem;
  color: #212529;
}
.carousel {
  position: relative;
}
.carousel-inner {
  position: relative;
  width: 100%;
  overflow: hidden;
}
.carousel-item {
  position: relative;
  display: none;
  -ms-flex-align: center;
      align-items: center;
  width: 100%;
  transition: -webkit-transform 0.6s ease;
  transition: transform 0.6s ease;
  transition: transform 0.6s ease, -webkit-transform 0.6s ease;
  -webkit-backface-visibility: hidden;
          backface-visibility: hidden;
  -webkit-perspective: 1000px;
          perspective: 1000px;
}
.carousel-item.active,
.carousel-item-next,
.carousel-item-prev {
  display: block;
}
.carousel-item-next,
.carousel-item-prev {
  position: absolute;
  top: 0;
}
.carousel-item-next.carousel-item-left,
.carousel-item-prev.carousel-item-right {
  -webkit-transform: translateX(0);
          transform: translateX(0);
}
@supports ((-webkit-transform-style: preserve-3d) or (transform-style: preserve-3d)) {
  .carousel-item-next.carousel-item-left,
  .carousel-item-prev.carousel-item-right {
    -webkit-transform: translate3d(0, 0, 0);
            transform: translate3d(0, 0, 0);
  }
}
.carousel-item-next,
.active.carousel-item-right {
  -webkit-transform: translateX(100%);
          transform: translateX(100%);
}
@supports ((-webkit-transform-style: preserve-3d) or (transform-style: preserve-3d)) {
  .carousel-item-next,
  .active.carousel-item-right {
    -webkit-transform: translate3d(100%, 0, 0);
            transform: translate3d(100%, 0, 0);
  }
}
.carousel-item-prev,
.active.carousel-item-left {
  -webkit-transform: translateX(-100%);
          transform: translateX(-100%);
}
@supports ((-webkit-transform-style: preserve-3d) or (transform-style: preserve-3d)) {
  .carousel-item-prev,
  .active.carousel-item-left {
    -webkit-transform: translate3d(-100%, 0, 0);
            transform: translate3d(-100%, 0, 0);
  }
}
.carousel-control-prev,
.carousel-control-next {
  position: absolute;
  top: 0;
  bottom: 0;
  display: -ms-flexbox;
  display: flex;
  -ms-flex-align: center;
      align-items: center;
  -ms-flex-pack: center;
      justify-content: center;
  width: 15%;
  color: #fff;
  text-align: center;
  opacity: 0.5;
}
.carousel-control-prev:focus, .carousel-control-prev:hover,
.carousel-control-next:focus,
.carousel-control-next:hover {
  color: #fff;
  text-decoration: none;
  outline: 0;
  opacity: .9;
}
.carousel-control-prev {
  left: 0;
}
.carousel-control-next {
  right: 0;
}
.carousel-control-prev-icon,
.carousel-control-next-icon {
  display: inline-block;
  width: 20px;
  height: 20px;
  background: transparent no-repeat center center;
  background-size: 100% 100%;
}
.carousel-control-prev-icon {
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='%23fff' viewBox='0 0 8 8'%3E%3Cpath d='M5.25 0l-4 4 4 4 1.5-1.5-2.5-2.5 2.5-2.5-1.5-1.5z'/%3E%3C/svg%3E&quot;);
}
.carousel-control-next-icon {
  background-image: url(&quot;data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='%23fff' viewBox='0 0 8 8'%3E%3Cpath d='M2.75 0l-1.5 1.5 2.5 2.5-2.5 2.5 1.5 1.5 4-4-4-4z'/%3E%3C/svg%3E&quot;);
}
.carousel-indicators {
  position: absolute;
  right: 0;
  bottom: 10px;
  left: 0;
  z-index: 15;
  display: -ms-flexbox;
  display: flex;
  -ms-flex-pack: center;
      justify-content: center;
  padding-left: 0;
  margin-right: 15%;
  margin-left: 15%;
  list-style: none;
}
.carousel-indicators li {
  position: relative;
  -ms-flex: 0 1 auto;
      flex: 0 1 auto;
  width: 30px;
  height: 3px;
  margin-right: 3px;
  margin-left: 3px;
  text-indent: -999px;
  background-color: rgba(255, 255, 255, 0.5);
}
.carousel-indicators li::before {
  position: absolute;
  top: -10px;
  left: 0;
  display: inline-block;
  width: 100%;
  height: 10px;
  content: &quot;&quot;;
}
.carousel-indicators li::after {
  position: absolute;
  bottom: -10px;
  left: 0;
  display: inline-block;
  width: 100%;
  height: 10px;
  content: &quot;&quot;;
}
.carousel-indicators .active {
  background-color: #fff;
}
.carousel-caption {
  position: absolute;
  right: 15%;
  bottom: 20px;
  left: 15%;
  z-index: 10;
  padding-top: 20px;
  padding-bottom: 20px;
  color: #fff;
  text-align: center;
}
.align-baseline {
  vertical-align: baseline !important;
}
.align-top {
  vertical-align: top !important;
}
.align-middle {
  vertical-align: middle !important;
}
.align-bottom {
  vertical-align: bottom !important;
}
.align-text-bottom {
  vertical-align: text-bottom !important;
}
.align-text-top {
  vertical-align: text-top !important;
}
.bg-primary {
  background-color: #007bff !important;
}
a.bg-primary:focus, a.bg-primary:hover {
  background-color: #0062cc !important;
}
.bg-secondary {
  background-color: #868e96 !important;
}
a.bg-secondary:focus, a.bg-secondary:hover {
  background-color: #6c757d !important;
}
.bg-success {
  background-color: #28a745 !important;
}
a.bg-success:focus, a.bg-success:hover {
  background-color: #1e7e34 !important;
}
.bg-info {
  background-color: #17a2b8 !important;
}
a.bg-info:focus, a.bg-info:hover {
  background-color: #117a8b !important;
}
.bg-warning {
  background-color: #ffc107 !important;
}
a.bg-warning:focus, a.bg-warning:hover {
  background-color: #d39e00 !important;
}
.bg-danger {
  background-color: #dc3545 !important;
}
a.bg-danger:focus, a.bg-danger:hover {
  background-color: #bd2130 !important;
}
.bg-light {
  background-color: #f8f9fa !important;
}
a.bg-light:focus, a.bg-light:hover {
  background-color: #dae0e5 !important;
}
.bg-dark {
  background-color: #343a40 !important;
}
a.bg-dark:focus, a.bg-dark:hover {
  background-color: #1d2124 !important;
}
.bg-white {
  background-color: #fff !important;
}
.bg-transparent {
  background-color: transparent !important;
}
.border {
  border: 1px solid #e9ecef !important;
}
.border-0 {
  border: 0 !important;
}
.border-top-0 {
  border-top: 0 !important;
}
.border-right-0 {
  border-right: 0 !important;
}
.border-bottom-0 {
  border-bottom: 0 !important;
}
.border-left-0 {
  border-left: 0 !important;
}
.border-primary {
  border-color: #007bff !important;
}
.border-secondary {
  border-color: #868e96 !important;
}
.border-success {
  border-color: #28a745 !important;
}
.border-info {
  border-color: #17a2b8 !important;
}
.border-warning {
  border-color: #ffc107 !important;
}
.border-danger {
  border-color: #dc3545 !important;
}
.border-light {
  border-color: #f8f9fa !important;
}
.border-dark {
  border-color: #343a40 !important;
}
.border-white {
  border-color: #fff !important;
}
.rounded {
  border-radius: 0.25rem !important;
}
.rounded-top {
  border-top-left-radius: 0.25rem !important;
  border-top-right-radius: 0.25rem !important;
}
.rounded-right {
  border-top-right-radius: 0.25rem !important;
  border-bottom-right-radius: 0.25rem !important;
}
.rounded-bottom {
  border-bottom-right-radius: 0.25rem !important;
  border-bottom-left-radius: 0.25rem !important;
}
.rounded-left {
  border-top-left-radius: 0.25rem !important;
  border-bottom-left-radius: 0.25rem !important;
}
.rounded-circle {
  border-radius: 50% !important;
}
.rounded-0 {
  border-radius: 0 !important;
}
.clearfix::after {
  display: block;
  clear: both;
  content: &quot;&quot;;
}
.d-none {
  display: none !important;
}
.d-inline {
  display: inline !important;
}
.d-inline-block {
  display: inline-block !important;
}
.d-block {
  display: block !important;
}
.d-table {
  display: table !important;
}
.d-table-row {
  display: table-row !important;
}
.d-table-cell {
  display: table-cell !important;
}
.d-flex {
  display: -ms-flexbox !important;
  display: flex !important;
}
.d-inline-flex {
  display: -ms-inline-flexbox !important;
  display: inline-flex !important;
}
@media (min-width: 576px) {
  .d-sm-none {
    display: none !important;
  }
  .d-sm-inline {
    display: inline !important;
  }
  .d-sm-inline-block {
    display: inline-block !important;
  }
  .d-sm-block {
    display: block !important;
  }
  .d-sm-table {
    display: table !important;
  }
  .d-sm-table-row {
    display: table-row !important;
  }
  .d-sm-table-cell {
    display: table-cell !important;
  }
  .d-sm-flex {
    display: -ms-flexbox !important;
    display: flex !important;
  }
  .d-sm-inline-flex {
    display: -ms-inline-flexbox !important;
    display: inline-flex !important;
  }
}
@media (min-width: 768px) {
  .d-md-none {
    display: none !important;
  }
  .d-md-inline {
    display: inline !important;
  }
  .d-md-inline-block {
    display: inline-block !important;
  }
  .d-md-block {
    display: block !important;
  }
  .d-md-table {
    display: table !important;
  }
  .d-md-table-row {
    display: table-row !important;
  }
  .d-md-table-cell {
    display: table-cell !important;
  }
  .d-md-flex {
    display: -ms-flexbox !important;
    display: flex !important;
  }
  .d-md-inline-flex {
    display: -ms-inline-flexbox !important;
    display: inline-flex !important;
  }
}
@media (min-width: 992px) {
  .d-lg-none {
    display: none !important;
  }
  .d-lg-inline {
    display: inline !important;
  }
  .d-lg-inline-block {
    display: inline-block !important;
  }
  .d-lg-block {
    display: block !important;
  }
  .d-lg-table {
    display: table !important;
  }
  .d-lg-table-row {
    display: table-row !important;
  }
  .d-lg-table-cell {
    display: table-cell !important;
  }
  .d-lg-flex {
    display: -ms-flexbox !important;
    display: flex !important;
  }
  .d-lg-inline-flex {
    display: -ms-inline-flexbox !important;
    display: inline-flex !important;
  }
}
@media (min-width: 1200px) {
  .d-xl-none {
    display: none !important;
  }
  .d-xl-inline {
    display: inline !important;
  }
  .d-xl-inline-block {
    display: inline-block !important;
  }
  .d-xl-block {
    display: block !important;
  }
  .d-xl-table {
    display: table !important;
  }
  .d-xl-table-row {
    display: table-row !important;
  }
  .d-xl-table-cell {
    display: table-cell !important;
  }
  .d-xl-flex {
    display: -ms-flexbox !important;
    display: flex !important;
  }
  .d-xl-inline-flex {
    display: -ms-inline-flexbox !important;
    display: inline-flex !important;
  }
}
.d-print-block {
  display: none !important;
}
@media print {
  .d-print-block {
    display: block !important;
  }
}
.d-print-inline {
  display: none !important;
}
@media print {
  .d-print-inline {
    display: inline !important;
  }
}
.d-print-inline-block {
  display: none !important;
}
@media print {
  .d-print-inline-block {
    display: inline-block !important;
  }
}
@media print {
  .d-print-none {
    display: none !important;
  }
}
.embed-responsive {
  position: relative;
  display: block;
  width: 100%;
  padding: 0;
  overflow: hidden;
}
.embed-responsive::before {
  display: block;
  content: &quot;&quot;;
}
.embed-responsive .embed-responsive-item,
.embed-responsive iframe,
.embed-responsive embed,
.embed-responsive object,
.embed-responsive video {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border: 0;
}
.embed-responsive-21by9::before {
  padding-top: 42.857143%;
}
.embed-responsive-16by9::before {
  padding-top: 56.25%;
}
.embed-responsive-4by3::before {
  padding-top: 75%;
}
.embed-responsive-1by1::before {
  padding-top: 100%;
}
.flex-row {
  -ms-flex-direction: row !important;
      flex-direction: row !important;
}
.flex-column {
  -ms-flex-direction: column !important;
      flex-direction: column !important;
}
.flex-row-reverse {
  -ms-flex-direction: row-reverse !important;
      flex-direction: row-reverse !important;
}
.flex-column-reverse {
  -ms-flex-direction: column-reverse !important;
      flex-direction: column-reverse !important;
}
.flex-wrap {
  -ms-flex-wrap: wrap !important;
      flex-wrap: wrap !important;
}
.flex-nowrap {
  -ms-flex-wrap: nowrap !important;
      flex-wrap: nowrap !important;
}
.flex-wrap-reverse {
  -ms-flex-wrap: wrap-reverse !important;
      flex-wrap: wrap-reverse !important;
}
.justify-content-start {
  -ms-flex-pack: start !important;
      justify-content: flex-start !important;
}
.justify-content-end {
  -ms-flex-pack: end !important;
      justify-content: flex-end !important;
}
.justify-content-center {
  -ms-flex-pack: center !important;
      justify-content: center !important;
}
.justify-content-between {
  -ms-flex-pack: justify !important;
      justify-content: space-between !important;
}
.justify-content-around {
  -ms-flex-pack: distribute !important;
      justify-content: space-around !important;
}
.align-items-start {
  -ms-flex-align: start !important;
      align-items: flex-start !important;
}
.align-items-end {
  -ms-flex-align: end !important;
      align-items: flex-end !important;
}
.align-items-center {
  -ms-flex-align: center !important;
      align-items: center !important;
}
.align-items-baseline {
  -ms-flex-align: baseline !important;
      align-items: baseline !important;
}
.align-items-stretch {
  -ms-flex-align: stretch !important;
      align-items: stretch !important;
}
.align-content-start {
  -ms-flex-line-pack: start !important;
      align-content: flex-start !important;
}
.align-content-end {
  -ms-flex-line-pack: end !important;
      align-content: flex-end !important;
}
.align-content-center {
  -ms-flex-line-pack: center !important;
      align-content: center !important;
}
.align-content-between {
  -ms-flex-line-pack: justify !important;
      align-content: space-between !important;
}
.align-content-around {
  -ms-flex-line-pack: distribute !important;
      align-content: space-around !important;
}
.align-content-stretch {
  -ms-flex-line-pack: stretch !important;
      align-content: stretch !important;
}
.align-self-auto {
  -ms-flex-item-align: auto !important;
      -ms-grid-row-align: auto !important;
      align-self: auto !important;
}
.align-self-start {
  -ms-flex-item-align: start !important;
      align-self: flex-start !important;
}
.align-self-end {
  -ms-flex-item-align: end !important;
      align-self: flex-end !important;
}
.align-self-center {
  -ms-flex-item-align: center !important;
      -ms-grid-row-align: center !important;
      align-self: center !important;
}
.align-self-baseline {
  -ms-flex-item-align: baseline !important;
      align-self: baseline !important;
}
.align-self-stretch {
  -ms-flex-item-align: stretch !important;
      -ms-grid-row-align: stretch !important;
      align-self: stretch !important;
}
@media (min-width: 576px) {
  .flex-sm-row {
    -ms-flex-direction: row !important;
        flex-direction: row !important;
  }
  .flex-sm-column {
    -ms-flex-direction: column !important;
        flex-direction: column !important;
  }
  .flex-sm-row-reverse {
    -ms-flex-direction: row-reverse !important;
        flex-direction: row-reverse !important;
  }
  .flex-sm-column-reverse {
    -ms-flex-direction: column-reverse !important;
        flex-direction: column-reverse !important;
  }
  .flex-sm-wrap {
    -ms-flex-wrap: wrap !important;
        flex-wrap: wrap !important;
  }
  .flex-sm-nowrap {
    -ms-flex-wrap: nowrap !important;
        flex-wrap: nowrap !important;
  }
  .flex-sm-wrap-reverse {
    -ms-flex-wrap: wrap-reverse !important;
        flex-wrap: wrap-reverse !important;
  }
  .justify-content-sm-start {
    -ms-flex-pack: start !important;
        justify-content: flex-start !important;
  }
  .justify-content-sm-end {
    -ms-flex-pack: end !important;
        justify-content: flex-end !important;
  }
  .justify-content-sm-center {
    -ms-flex-pack: center !important;
        justify-content: center !important;
  }
  .justify-content-sm-between {
    -ms-flex-pack: justify !important;
        justify-content: space-between !important;
  }
  .justify-content-sm-around {
    -ms-flex-pack: distribute !important;
        justify-content: space-around !important;
  }
  .align-items-sm-start {
    -ms-flex-align: start !important;
        align-items: flex-start !important;
  }
  .align-items-sm-end {
    -ms-flex-align: end !important;
        align-items: flex-end !important;
  }
  .align-items-sm-center {
    -ms-flex-align: center !important;
        align-items: center !important;
  }
  .align-items-sm-baseline {
    -ms-flex-align: baseline !important;
        align-items: baseline !important;
  }
  .align-items-sm-stretch {
    -ms-flex-align: stretch !important;
        align-items: stretch !important;
  }
  .align-content-sm-start {
    -ms-flex-line-pack: start !important;
        align-content: flex-start !important;
  }
  .align-content-sm-end {
    -ms-flex-line-pack: end !important;
        align-content: flex-end !important;
  }
  .align-content-sm-center {
    -ms-flex-line-pack: center !important;
        align-content: center !important;
  }
  .align-content-sm-between {
    -ms-flex-line-pack: justify !important;
        align-content: space-between !important;
  }
  .align-content-sm-around {
    -ms-flex-line-pack: distribute !important;
        align-content: space-around !important;
  }
  .align-content-sm-stretch {
    -ms-flex-line-pack: stretch !important;
        align-content: stretch !important;
  }
  .align-self-sm-auto {
    -ms-flex-item-align: auto !important;
        -ms-grid-row-align: auto !important;
        align-self: auto !important;
  }
  .align-self-sm-start {
    -ms-flex-item-align: start !important;
        align-self: flex-start !important;
  }
  .align-self-sm-end {
    -ms-flex-item-align: end !important;
        align-self: flex-end !important;
  }
  .align-self-sm-center {
    -ms-flex-item-align: center !important;
        -ms-grid-row-align: center !important;
        align-self: center !important;
  }
  .align-self-sm-baseline {
    -ms-flex-item-align: baseline !important;
        align-self: baseline !important;
  }
  .align-self-sm-stretch {
    -ms-flex-item-align: stretch !important;
        -ms-grid-row-align: stretch !important;
        align-self: stretch !important;
  }
}
@media (min-width: 768px) {
  .flex-md-row {
    -ms-flex-direction: row !important;
        flex-direction: row !important;
  }
  .flex-md-column {
    -ms-flex-direction: column !important;
        flex-direction: column !important;
  }
  .flex-md-row-reverse {
    -ms-flex-direction: row-reverse !important;
        flex-direction: row-reverse !important;
  }
  .flex-md-column-reverse {
    -ms-flex-direction: column-reverse !important;
        flex-direction: column-reverse !important;
  }
  .flex-md-wrap {
    -ms-flex-wrap: wrap !important;
        flex-wrap: wrap !important;
  }
  .flex-md-nowrap {
    -ms-flex-wrap: nowrap !important;
        flex-wrap: nowrap !important;
  }
  .flex-md-wrap-reverse {
    -ms-flex-wrap: wrap-reverse !important;
        flex-wrap: wrap-reverse !important;
  }
  .justify-content-md-start {
    -ms-flex-pack: start !important;
        justify-content: flex-start !important;
  }
  .justify-content-md-end {
    -ms-flex-pack: end !important;
        justify-content: flex-end !important;
  }
  .justify-content-md-center {
    -ms-flex-pack: center !important;
        justify-content: center !important;
  }
  .justify-content-md-between {
    -ms-flex-pack: justify !important;
        justify-content: space-between !important;
  }
  .justify-content-md-around {
    -ms-flex-pack: distribute !important;
        justify-content: space-around !important;
  }
  .align-items-md-start {
    -ms-flex-align: start !important;
        align-items: flex-start !important;
  }
  .align-items-md-end {
    -ms-flex-align: end !important;
        align-items: flex-end !important;
  }
  .align-items-md-center {
    -ms-flex-align: center !important;
        align-items: center !important;
  }
  .align-items-md-baseline {
    -ms-flex-align: baseline !important;
        align-items: baseline !important;
  }
  .align-items-md-stretch {
    -ms-flex-align: stretch !important;
        align-items: stretch !important;
  }
  .align-content-md-start {
    -ms-flex-line-pack: start !important;
        align-content: flex-start !important;
  }
  .align-content-md-end {
    -ms-flex-line-pack: end !important;
        align-content: flex-end !important;
  }
  .align-content-md-center {
    -ms-flex-line-pack: center !important;
        align-content: center !important;
  }
  .align-content-md-between {
    -ms-flex-line-pack: justify !important;
        align-content: space-between !important;
  }
  .align-content-md-around {
    -ms-flex-line-pack: distribute !important;
        align-content: space-around !important;
  }
  .align-content-md-stretch {
    -ms-flex-line-pack: stretch !important;
        align-content: stretch !important;
  }
  .align-self-md-auto {
    -ms-flex-item-align: auto !important;
        -ms-grid-row-align: auto !important;
        align-self: auto !important;
  }
  .align-self-md-start {
    -ms-flex-item-align: start !important;
        align-self: flex-start !important;
  }
  .align-self-md-end {
    -ms-flex-item-align: end !important;
        align-self: flex-end !important;
  }
  .align-self-md-center {
    -ms-flex-item-align: center !important;
        -ms-grid-row-align: center !important;
        align-self: center !important;
  }
  .align-self-md-baseline {
    -ms-flex-item-align: baseline !important;
        align-self: baseline !important;
  }
  .align-self-md-stretch {
    -ms-flex-item-align: stretch !important;
        -ms-grid-row-align: stretch !important;
        align-self: stretch !important;
  }
}
@media (min-width: 992px) {
  .flex-lg-row {
    -ms-flex-direction: row !important;
        flex-direction: row !important;
  }
  .flex-lg-column {
    -ms-flex-direction: column !important;
        flex-direction: column !important;
  }
  .flex-lg-row-reverse {
    -ms-flex-direction: row-reverse !important;
        flex-direction: row-reverse !important;
  }
  .flex-lg-column-reverse {
    -ms-flex-direction: column-reverse !important;
        flex-direction: column-reverse !important;
  }
  .flex-lg-wrap {
    -ms-flex-wrap: wrap !important;
        flex-wrap: wrap !important;
  }
  .flex-lg-nowrap {
    -ms-flex-wrap: nowrap !important;
        flex-wrap: nowrap !important;
  }
  .flex-lg-wrap-reverse {
    -ms-flex-wrap: wrap-reverse !important;
        flex-wrap: wrap-reverse !important;
  }
  .justify-content-lg-start {
    -ms-flex-pack: start !important;
        justify-content: flex-start !important;
  }
  .justify-content-lg-end {
    -ms-flex-pack: end !important;
        justify-content: flex-end !important;
  }
  .justify-content-lg-center {
    -ms-flex-pack: center !important;
        justify-content: center !important;
  }
  .justify-content-lg-between {
    -ms-flex-pack: justify !important;
        justify-content: space-between !important;
  }
  .justify-content-lg-around {
    -ms-flex-pack: distribute !important;
        justify-content: space-around !important;
  }
  .align-items-lg-start {
    -ms-flex-align: start !important;
        align-items: flex-start !important;
  }
  .align-items-lg-end {
    -ms-flex-align: end !important;
        align-items: flex-end !important;
  }
  .align-items-lg-center {
    -ms-flex-align: center !important;
        align-items: center !important;
  }
  .align-items-lg-baseline {
    -ms-flex-align: baseline !important;
        align-items: baseline !important;
  }
  .align-items-lg-stretch {
    -ms-flex-align: stretch !important;
        align-items: stretch !important;
  }
  .align-content-lg-start {
    -ms-flex-line-pack: start !important;
        align-content: flex-start !important;
  }
  .align-content-lg-end {
    -ms-flex-line-pack: end !important;
        align-content: flex-end !important;
  }
  .align-content-lg-center {
    -ms-flex-line-pack: center !important;
        align-content: center !important;
  }
  .align-content-lg-between {
    -ms-flex-line-pack: justify !important;
        align-content: space-between !important;
  }
  .align-content-lg-around {
    -ms-flex-line-pack: distribute !important;
        align-content: space-around !important;
  }
  .align-content-lg-stretch {
    -ms-flex-line-pack: stretch !important;
        align-content: stretch !important;
  }
  .align-self-lg-auto {
    -ms-flex-item-align: auto !important;
        -ms-grid-row-align: auto !important;
        align-self: auto !important;
  }
  .align-self-lg-start {
    -ms-flex-item-align: start !important;
        align-self: flex-start !important;
  }
  .align-self-lg-end {
    -ms-flex-item-align: end !important;
        align-self: flex-end !important;
  }
  .align-self-lg-center {
    -ms-flex-item-align: center !important;
        -ms-grid-row-align: center !important;
        align-self: center !important;
  }
  .align-self-lg-baseline {
    -ms-flex-item-align: baseline !important;
        align-self: baseline !important;
  }
  .align-self-lg-stretch {
    -ms-flex-item-align: stretch !important;
        -ms-grid-row-align: stretch !important;
        align-self: stretch !important;
  }
}
@media (min-width: 1200px) {
  .flex-xl-row {
    -ms-flex-direction: row !important;
        flex-direction: row !important;
  }
  .flex-xl-column {
    -ms-flex-direction: column !important;
        flex-direction: column !important;
  }
  .flex-xl-row-reverse {
    -ms-flex-direction: row-reverse !important;
        flex-direction: row-reverse !important;
  }
  .flex-xl-column-reverse {
    -ms-flex-direction: column-reverse !important;
        flex-direction: column-reverse !important;
  }
  .flex-xl-wrap {
    -ms-flex-wrap: wrap !important;
        flex-wrap: wrap !important;
  }
  .flex-xl-nowrap {
    -ms-flex-wrap: nowrap !important;
        flex-wrap: nowrap !important;
  }
  .flex-xl-wrap-reverse {
    -ms-flex-wrap: wrap-reverse !important;
        flex-wrap: wrap-reverse !important;
  }
  .justify-content-xl-start {
    -ms-flex-pack: start !important;
        justify-content: flex-start !important;
  }
  .justify-content-xl-end {
    -ms-flex-pack: end !important;
        justify-content: flex-end !important;
  }
  .justify-content-xl-center {
    -ms-flex-pack: center !important;
        justify-content: center !important;
  }
  .justify-content-xl-between {
    -ms-flex-pack: justify !important;
        justify-content: space-between !important;
  }
  .justify-content-xl-around {
    -ms-flex-pack: distribute !important;
        justify-content: space-around !important;
  }
  .align-items-xl-start {
    -ms-flex-align: start !important;
        align-items: flex-start !important;
  }
  .align-items-xl-end {
    -ms-flex-align: end !important;
        align-items: flex-end !important;
  }
  .align-items-xl-center {
    -ms-flex-align: center !important;
        align-items: center !important;
  }
  .align-items-xl-baseline {
    -ms-flex-align: baseline !important;
        align-items: baseline !important;
  }
  .align-items-xl-stretch {
    -ms-flex-align: stretch !important;
        align-items: stretch !important;
  }
  .align-content-xl-start {
    -ms-flex-line-pack: start !important;
        align-content: flex-start !important;
  }
  .align-content-xl-end {
    -ms-flex-line-pack: end !important;
        align-content: flex-end !important;
  }
  .align-content-xl-center {
    -ms-flex-line-pack: center !important;
        align-content: center !important;
  }
  .align-content-xl-between {
    -ms-flex-line-pack: justify !important;
        align-content: space-between !important;
  }
  .align-content-xl-around {
    -ms-flex-line-pack: distribute !important;
        align-content: space-around !important;
  }
  .align-content-xl-stretch {
    -ms-flex-line-pack: stretch !important;
        align-content: stretch !important;
  }
  .align-self-xl-auto {
    -ms-flex-item-align: auto !important;
        -ms-grid-row-align: auto !important;
        align-self: auto !important;
  }
  .align-self-xl-start {
    -ms-flex-item-align: start !important;
        align-self: flex-start !important;
  }
  .align-self-xl-end {
    -ms-flex-item-align: end !important;
        align-self: flex-end !important;
  }
  .align-self-xl-center {
    -ms-flex-item-align: center !important;
        -ms-grid-row-align: center !important;
        align-self: center !important;
  }
  .align-self-xl-baseline {
    -ms-flex-item-align: baseline !important;
        align-self: baseline !important;
  }
  .align-self-xl-stretch {
    -ms-flex-item-align: stretch !important;
        -ms-grid-row-align: stretch !important;
        align-self: stretch !important;
  }
}
.float-left {
  float: left !important;
}
.float-right {
  float: right !important;
}
.float-none {
  float: none !important;
}
@media (min-width: 576px) {
  .float-sm-left {
    float: left !important;
  }
  .float-sm-right {
    float: right !important;
  }
  .float-sm-none {
    float: none !important;
  }
}
@media (min-width: 768px) {
  .float-md-left {
    float: left !important;
  }
  .float-md-right {
    float: right !important;
  }
  .float-md-none {
    float: none !important;
  }
}
@media (min-width: 992px) {
  .float-lg-left {
    float: left !important;
  }
  .float-lg-right {
    float: right !important;
  }
  .float-lg-none {
    float: none !important;
  }
}
@media (min-width: 1200px) {
  .float-xl-left {
    float: left !important;
  }
  .float-xl-right {
    float: right !important;
  }
  .float-xl-none {
    float: none !important;
  }
}
.position-static {
  position: static !important;
}
.position-relative {
  position: relative !important;
}
.position-absolute {
  position: absolute !important;
}
.position-fixed {
  position: fixed !important;
}
.position-sticky {
  position: -webkit-sticky !important;
  position: sticky !important;
}
.fixed-top {
  position: fixed;
  top: 0;
  right: 0;
  left: 0;
  z-index: 1030;
}
.fixed-bottom {
  position: fixed;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 1030;
}
@supports ((position: -webkit-sticky) or (position: sticky)) {
  .sticky-top {
    position: -webkit-sticky;
    position: sticky;
    top: 0;
    z-index: 1020;
  }
}
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  -webkit-clip-path: inset(50%);
          clip-path: inset(50%);
  border: 0;
}
.sr-only-focusable:active, .sr-only-focusable:focus {
  position: static;
  width: auto;
  height: auto;
  overflow: visible;
  clip: auto;
  white-space: normal;
  -webkit-clip-path: none;
          clip-path: none;
}
.w-25 {
  width: 25% !important;
}
.w-50 {
  width: 50% !important;
}
.w-75 {
  width: 75% !important;
}
.w-100 {
  width: 100% !important;
}
.h-25 {
  height: 25% !important;
}
.h-50 {
  height: 50% !important;
}
.h-75 {
  height: 75% !important;
}
.h-100 {
  height: 100% !important;
}
.mw-100 {
  max-width: 100% !important;
}
.mh-100 {
  max-height: 100% !important;
}
.m-0 {
  margin: 0 !important;
}
.mt-0,
.my-0 {
  margin-top: 0 !important;
}
.mr-0,
.mx-0 {
  margin-right: 0 !important;
}
.mb-0,
.my-0 {
  margin-bottom: 0 !important;
}
.ml-0,
.mx-0 {
  margin-left: 0 !important;
}
.m-1 {
  margin: 0.25rem !important;
}
.mt-1,
.my-1 {
  margin-top: 0.25rem !important;
}
.mr-1,
.mx-1 {
  margin-right: 0.25rem !important;
}
.mb-1,
.my-1 {
  margin-bottom: 0.25rem !important;
}
.ml-1,
.mx-1 {
  margin-left: 0.25rem !important;
}
.m-2 {
  margin: 0.5rem !important;
}
.mt-2,
.my-2 {
  margin-top: 0.5rem !important;
}
.mr-2,
.mx-2 {
  margin-right: 0.5rem !important;
}
.mb-2,
.my-2 {
  margin-bottom: 0.5rem !important;
}
.ml-2,
.mx-2 {
  margin-left: 0.5rem !important;
}
.m-3 {
  margin: 1rem !important;
}
.mt-3,
.my-3 {
  margin-top: 1rem !important;
}
.mr-3,
.mx-3 {
  margin-right: 1rem !important;
}
.mb-3,
.my-3 {
  margin-bottom: 1rem !important;
}
.ml-3,
.mx-3 {
  margin-left: 1rem !important;
}
.m-4 {
  margin: 1.5rem !important;
}
.mt-4,
.my-4 {
  margin-top: 1.5rem !important;
}
.mr-4,
.mx-4 {
  margin-right: 1.5rem !important;
}
.mb-4,
.my-4 {
  margin-bottom: 1.5rem !important;
}
.ml-4,
.mx-4 {
  margin-left: 1.5rem !important;
}
.m-5 {
  margin: 3rem !important;
}
.mt-5,
.my-5 {
  margin-top: 3rem !important;
}
.mr-5,
.mx-5 {
  margin-right: 3rem !important;
}
.mb-5,
.my-5 {
  margin-bottom: 3rem !important;
}
.ml-5,
.mx-5 {
  margin-left: 3rem !important;
}
.p-0 {
  padding: 0 !important;
}
.pt-0,
.py-0 {
  padding-top: 0 !important;
}
.pr-0,
.px-0 {
  padding-right: 0 !important;
}
.pb-0,
.py-0 {
  padding-bottom: 0 !important;
}
.pl-0,
.px-0 {
  padding-left: 0 !important;
}
.p-1 {
  padding: 0.25rem !important;
}
.pt-1,
.py-1 {
  padding-top: 0.25rem !important;
}
.pr-1,
.px-1 {
  padding-right: 0.25rem !important;
}
.pb-1,
.py-1 {
  padding-bottom: 0.25rem !important;
}
.pl-1,
.px-1 {
  padding-left: 0.25rem !important;
}
.p-2 {
  padding: 0.5rem !important;
}
.pt-2,
.py-2 {
  padding-top: 0.5rem !important;
}
.pr-2,
.px-2 {
  padding-right: 0.5rem !important;
}
.pb-2,
.py-2 {
  padding-bottom: 0.5rem !important;
}
.pl-2,
.px-2 {
  padding-left: 0.5rem !important;
}
.p-3 {
  padding: 1rem !important;
}
.pt-3,
.py-3 {
  padding-top: 1rem !important;
}
.pr-3,
.px-3 {
  padding-right: 1rem !important;
}
.pb-3,
.py-3 {
  padding-bottom: 1rem !important;
}
.pl-3,
.px-3 {
  padding-left: 1rem !important;
}
.p-4 {
  padding: 1.5rem !important;
}
.pt-4,
.py-4 {
  padding-top: 1.5rem !important;
}
.pr-4,
.px-4 {
  padding-right: 1.5rem !important;
}
.pb-4,
.py-4 {
  padding-bottom: 1.5rem !important;
}
.pl-4,
.px-4 {
  padding-left: 1.5rem !important;
}
.p-5 {
  padding: 3rem !important;
}
.pt-5,
.py-5 {
  padding-top: 3rem !important;
}
.pr-5,
.px-5 {
  padding-right: 3rem !important;
}
.pb-5,
.py-5 {
  padding-bottom: 3rem !important;
}
.pl-5,
.px-5 {
  padding-left: 3rem !important;
}
.m-auto {
  margin: auto !important;
}
.mt-auto,
.my-auto {
  margin-top: auto !important;
}
.mr-auto,
.mx-auto {
  margin-right: auto !important;
}
.mb-auto,
.my-auto {
  margin-bottom: auto !important;
}
.ml-auto,
.mx-auto {
  margin-left: auto !important;
}
@media (min-width: 576px) {
  .m-sm-0 {
    margin: 0 !important;
  }
  .mt-sm-0,
  .my-sm-0 {
    margin-top: 0 !important;
  }
  .mr-sm-0,
  .mx-sm-0 {
    margin-right: 0 !important;
  }
  .mb-sm-0,
  .my-sm-0 {
    margin-bottom: 0 !important;
  }
  .ml-sm-0,
  .mx-sm-0 {
    margin-left: 0 !important;
  }
  .m-sm-1 {
    margin: 0.25rem !important;
  }
  .mt-sm-1,
  .my-sm-1 {
    margin-top: 0.25rem !important;
  }
  .mr-sm-1,
  .mx-sm-1 {
    margin-right: 0.25rem !important;
  }
  .mb-sm-1,
  .my-sm-1 {
    margin-bottom: 0.25rem !important;
  }
  .ml-sm-1,
  .mx-sm-1 {
    margin-left: 0.25rem !important;
  }
  .m-sm-2 {
    margin: 0.5rem !important;
  }
  .mt-sm-2,
  .my-sm-2 {
    margin-top: 0.5rem !important;
  }
  .mr-sm-2,
  .mx-sm-2 {
    margin-right: 0.5rem !important;
  }
  .mb-sm-2,
  .my-sm-2 {
    margin-bottom: 0.5rem !important;
  }
  .ml-sm-2,
  .mx-sm-2 {
    margin-left: 0.5rem !important;
  }
  .m-sm-3 {
    margin: 1rem !important;
  }
  .mt-sm-3,
  .my-sm-3 {
    margin-top: 1rem !important;
  }
  .mr-sm-3,
  .mx-sm-3 {
    margin-right: 1rem !important;
  }
  .mb-sm-3,
  .my-sm-3 {
    margin-bottom: 1rem !important;
  }
  .ml-sm-3,
  .mx-sm-3 {
    margin-left: 1rem !important;
  }
  .m-sm-4 {
    margin: 1.5rem !important;
  }
  .mt-sm-4,
  .my-sm-4 {
    margin-top: 1.5rem !important;
  }
  .mr-sm-4,
  .mx-sm-4 {
    margin-right: 1.5rem !important;
  }
  .mb-sm-4,
  .my-sm-4 {
    margin-bottom: 1.5rem !important;
  }
  .ml-sm-4,
  .mx-sm-4 {
    margin-left: 1.5rem !important;
  }
  .m-sm-5 {
    margin: 3rem !important;
  }
  .mt-sm-5,
  .my-sm-5 {
    margin-top: 3rem !important;
  }
  .mr-sm-5,
  .mx-sm-5 {
    margin-right: 3rem !important;
  }
  .mb-sm-5,
  .my-sm-5 {
    margin-bottom: 3rem !important;
  }
  .ml-sm-5,
  .mx-sm-5 {
    margin-left: 3rem !important;
  }
  .p-sm-0 {
    padding: 0 !important;
  }
  .pt-sm-0,
  .py-sm-0 {
    padding-top: 0 !important;
  }
  .pr-sm-0,
  .px-sm-0 {
    padding-right: 0 !important;
  }
  .pb-sm-0,
  .py-sm-0 {
    padding-bottom: 0 !important;
  }
  .pl-sm-0,
  .px-sm-0 {
    padding-left: 0 !important;
  }
  .p-sm-1 {
    padding: 0.25rem !important;
  }
  .pt-sm-1,
  .py-sm-1 {
    padding-top: 0.25rem !important;
  }
  .pr-sm-1,
  .px-sm-1 {
    padding-right: 0.25rem !important;
  }
  .pb-sm-1,
  .py-sm-1 {
    padding-bottom: 0.25rem !important;
  }
  .pl-sm-1,
  .px-sm-1 {
    padding-left: 0.25rem !important;
  }
  .p-sm-2 {
    padding: 0.5rem !important;
  }
  .pt-sm-2,
  .py-sm-2 {
    padding-top: 0.5rem !important;
  }
  .pr-sm-2,
  .px-sm-2 {
    padding-right: 0.5rem !important;
  }
  .pb-sm-2,
  .py-sm-2 {
    padding-bottom: 0.5rem !important;
  }
  .pl-sm-2,
  .px-sm-2 {
    padding-left: 0.5rem !important;
  }
  .p-sm-3 {
    padding: 1rem !important;
  }
  .pt-sm-3,
  .py-sm-3 {
    padding-top: 1rem !important;
  }
  .pr-sm-3,
  .px-sm-3 {
    padding-right: 1rem !important;
  }
  .pb-sm-3,
  .py-sm-3 {
    padding-bottom: 1rem !important;
  }
  .pl-sm-3,
  .px-sm-3 {
    padding-left: 1rem !important;
  }
  .p-sm-4 {
    padding: 1.5rem !important;
  }
  .pt-sm-4,
  .py-sm-4 {
    padding-top: 1.5rem !important;
  }
  .pr-sm-4,
  .px-sm-4 {
    padding-right: 1.5rem !important;
  }
  .pb-sm-4,
  .py-sm-4 {
    padding-bottom: 1.5rem !important;
  }
  .pl-sm-4,
  .px-sm-4 {
    padding-left: 1.5rem !important;
  }
  .p-sm-5 {
    padding: 3rem !important;
  }
  .pt-sm-5,
  .py-sm-5 {
    padding-top: 3rem !important;
  }
  .pr-sm-5,
  .px-sm-5 {
    padding-right: 3rem !important;
  }
  .pb-sm-5,
  .py-sm-5 {
    padding-bottom: 3rem !important;
  }
  .pl-sm-5,
  .px-sm-5 {
    padding-left: 3rem !important;
  }
  .m-sm-auto {
    margin: auto !important;
  }
  .mt-sm-auto,
  .my-sm-auto {
    margin-top: auto !important;
  }
  .mr-sm-auto,
  .mx-sm-auto {
    margin-right: auto !important;
  }
  .mb-sm-auto,
  .my-sm-auto {
    margin-bottom: auto !important;
  }
  .ml-sm-auto,
  .mx-sm-auto {
    margin-left: auto !important;
  }
}
@media (min-width: 768px) {
  .m-md-0 {
    margin: 0 !important;
  }
  .mt-md-0,
  .my-md-0 {
    margin-top: 0 !important;
  }
  .mr-md-0,
  .mx-md-0 {
    margin-right: 0 !important;
  }
  .mb-md-0,
  .my-md-0 {
    margin-bottom: 0 !important;
  }
  .ml-md-0,
  .mx-md-0 {
    margin-left: 0 !important;
  }
  .m-md-1 {
    margin: 0.25rem !important;
  }
  .mt-md-1,
  .my-md-1 {
    margin-top: 0.25rem !important;
  }
  .mr-md-1,
  .mx-md-1 {
    margin-right: 0.25rem !important;
  }
  .mb-md-1,
  .my-md-1 {
    margin-bottom: 0.25rem !important;
  }
  .ml-md-1,
  .mx-md-1 {
    margin-left: 0.25rem !important;
  }
  .m-md-2 {
    margin: 0.5rem !important;
  }
  .mt-md-2,
  .my-md-2 {
    margin-top: 0.5rem !important;
  }
  .mr-md-2,
  .mx-md-2 {
    margin-right: 0.5rem !important;
  }
  .mb-md-2,
  .my-md-2 {
    margin-bottom: 0.5rem !important;
  }
  .ml-md-2,
  .mx-md-2 {
    margin-left: 0.5rem !important;
  }
  .m-md-3 {
    margin: 1rem !important;
  }
  .mt-md-3,
  .my-md-3 {
    margin-top: 1rem !important;
  }
  .mr-md-3,
  .mx-md-3 {
    margin-right: 1rem !important;
  }
  .mb-md-3,
  .my-md-3 {
    margin-bottom: 1rem !important;
  }
  .ml-md-3,
  .mx-md-3 {
    margin-left: 1rem !important;
  }
  .m-md-4 {
    margin: 1.5rem !important;
  }
  .mt-md-4,
  .my-md-4 {
    margin-top: 1.5rem !important;
  }
  .mr-md-4,
  .mx-md-4 {
    margin-right: 1.5rem !important;
  }
  .mb-md-4,
  .my-md-4 {
    margin-bottom: 1.5rem !important;
  }
  .ml-md-4,
  .mx-md-4 {
    margin-left: 1.5rem !important;
  }
  .m-md-5 {
    margin: 3rem !important;
  }
  .mt-md-5,
  .my-md-5 {
    margin-top: 3rem !important;
  }
  .mr-md-5,
  .mx-md-5 {
    margin-right: 3rem !important;
  }
  .mb-md-5,
  .my-md-5 {
    margin-bottom: 3rem !important;
  }
  .ml-md-5,
  .mx-md-5 {
    margin-left: 3rem !important;
  }
  .p-md-0 {
    padding: 0 !important;
  }
  .pt-md-0,
  .py-md-0 {
    padding-top: 0 !important;
  }
  .pr-md-0,
  .px-md-0 {
    padding-right: 0 !important;
  }
  .pb-md-0,
  .py-md-0 {
    padding-bottom: 0 !important;
  }
  .pl-md-0,
  .px-md-0 {
    padding-left: 0 !important;
  }
  .p-md-1 {
    padding: 0.25rem !important;
  }
  .pt-md-1,
  .py-md-1 {
    padding-top: 0.25rem !important;
  }
  .pr-md-1,
  .px-md-1 {
    padding-right: 0.25rem !important;
  }
  .pb-md-1,
  .py-md-1 {
    padding-bottom: 0.25rem !important;
  }
  .pl-md-1,
  .px-md-1 {
    padding-left: 0.25rem !important;
  }
  .p-md-2 {
    padding: 0.5rem !important;
  }
  .pt-md-2,
  .py-md-2 {
    padding-top: 0.5rem !important;
  }
  .pr-md-2,
  .px-md-2 {
    padding-right: 0.5rem !important;
  }
  .pb-md-2,
  .py-md-2 {
    padding-bottom: 0.5rem !important;
  }
  .pl-md-2,
  .px-md-2 {
    padding-left: 0.5rem !important;
  }
  .p-md-3 {
    padding: 1rem !important;
  }
  .pt-md-3,
  .py-md-3 {
    padding-top: 1rem !important;
  }
  .pr-md-3,
  .px-md-3 {
    padding-right: 1rem !important;
  }
  .pb-md-3,
  .py-md-3 {
    padding-bottom: 1rem !important;
  }
  .pl-md-3,
  .px-md-3 {
    padding-left: 1rem !important;
  }
  .p-md-4 {
    padding: 1.5rem !important;
  }
  .pt-md-4,
  .py-md-4 {
    padding-top: 1.5rem !important;
  }
  .pr-md-4,
  .px-md-4 {
    padding-right: 1.5rem !important;
  }
  .pb-md-4,
  .py-md-4 {
    padding-bottom: 1.5rem !important;
  }
  .pl-md-4,
  .px-md-4 {
    padding-left: 1.5rem !important;
  }
  .p-md-5 {
    padding: 3rem !important;
  }
  .pt-md-5,
  .py-md-5 {
    padding-top: 3rem !important;
  }
  .pr-md-5,
  .px-md-5 {
    padding-right: 3rem !important;
  }
  .pb-md-5,
  .py-md-5 {
    padding-bottom: 3rem !important;
  }
  .pl-md-5,
  .px-md-5 {
    padding-left: 3rem !important;
  }
  .m-md-auto {
    margin: auto !important;
  }
  .mt-md-auto,
  .my-md-auto {
    margin-top: auto !important;
  }
  .mr-md-auto,
  .mx-md-auto {
    margin-right: auto !important;
  }
  .mb-md-auto,
  .my-md-auto {
    margin-bottom: auto !important;
  }
  .ml-md-auto,
  .mx-md-auto {
    margin-left: auto !important;
  }
}
@media (min-width: 992px) {
  .m-lg-0 {
    margin: 0 !important;
  }
  .mt-lg-0,
  .my-lg-0 {
    margin-top: 0 !important;
  }
  .mr-lg-0,
  .mx-lg-0 {
    margin-right: 0 !important;
  }
  .mb-lg-0,
  .my-lg-0 {
    margin-bottom: 0 !important;
  }
  .ml-lg-0,
  .mx-lg-0 {
    margin-left: 0 !important;
  }
  .m-lg-1 {
    margin: 0.25rem !important;
  }
  .mt-lg-1,
  .my-lg-1 {
    margin-top: 0.25rem !important;
  }
  .mr-lg-1,
  .mx-lg-1 {
    margin-right: 0.25rem !important;
  }
  .mb-lg-1,
  .my-lg-1 {
    margin-bottom: 0.25rem !important;
  }
  .ml-lg-1,
  .mx-lg-1 {
    margin-left: 0.25rem !important;
  }
  .m-lg-2 {
    margin: 0.5rem !important;
  }
  .mt-lg-2,
  .my-lg-2 {
    margin-top: 0.5rem !important;
  }
  .mr-lg-2,
  .mx-lg-2 {
    margin-right: 0.5rem !important;
  }
  .mb-lg-2,
  .my-lg-2 {
    margin-bottom: 0.5rem !important;
  }
  .ml-lg-2,
  .mx-lg-2 {
    margin-left: 0.5rem !important;
  }
  .m-lg-3 {
    margin: 1rem !important;
  }
  .mt-lg-3,
  .my-lg-3 {
    margin-top: 1rem !important;
  }
  .mr-lg-3,
  .mx-lg-3 {
    margin-right: 1rem !important;
  }
  .mb-lg-3,
  .my-lg-3 {
    margin-bottom: 1rem !important;
  }
  .ml-lg-3,
  .mx-lg-3 {
    margin-left: 1rem !important;
  }
  .m-lg-4 {
    margin: 1.5rem !important;
  }
  .mt-lg-4,
  .my-lg-4 {
    margin-top: 1.5rem !important;
  }
  .mr-lg-4,
  .mx-lg-4 {
    margin-right: 1.5rem !important;
  }
  .mb-lg-4,
  .my-lg-4 {
    margin-bottom: 1.5rem !important;
  }
  .ml-lg-4,
  .mx-lg-4 {
    margin-left: 1.5rem !important;
  }
  .m-lg-5 {
    margin: 3rem !important;
  }
  .mt-lg-5,
  .my-lg-5 {
    margin-top: 3rem !important;
  }
  .mr-lg-5,
  .mx-lg-5 {
    margin-right: 3rem !important;
  }
  .mb-lg-5,
  .my-lg-5 {
    margin-bottom: 3rem !important;
  }
  .ml-lg-5,
  .mx-lg-5 {
    margin-left: 3rem !important;
  }
  .p-lg-0 {
    padding: 0 !important;
  }
  .pt-lg-0,
  .py-lg-0 {
    padding-top: 0 !important;
  }
  .pr-lg-0,
  .px-lg-0 {
    padding-right: 0 !important;
  }
  .pb-lg-0,
  .py-lg-0 {
    padding-bottom: 0 !important;
  }
  .pl-lg-0,
  .px-lg-0 {
    padding-left: 0 !important;
  }
  .p-lg-1 {
    padding: 0.25rem !important;
  }
  .pt-lg-1,
  .py-lg-1 {
    padding-top: 0.25rem !important;
  }
  .pr-lg-1,
  .px-lg-1 {
    padding-right: 0.25rem !important;
  }
  .pb-lg-1,
  .py-lg-1 {
    padding-bottom: 0.25rem !important;
  }
  .pl-lg-1,
  .px-lg-1 {
    padding-left: 0.25rem !important;
  }
  .p-lg-2 {
    padding: 0.5rem !important;
  }
  .pt-lg-2,
  .py-lg-2 {
    padding-top: 0.5rem !important;
  }
  .pr-lg-2,
  .px-lg-2 {
    padding-right: 0.5rem !important;
  }
  .pb-lg-2,
  .py-lg-2 {
    padding-bottom: 0.5rem !important;
  }
  .pl-lg-2,
  .px-lg-2 {
    padding-left: 0.5rem !important;
  }
  .p-lg-3 {
    padding: 1rem !important;
  }
  .pt-lg-3,
  .py-lg-3 {
    padding-top: 1rem !important;
  }
  .pr-lg-3,
  .px-lg-3 {
    padding-right: 1rem !important;
  }
  .pb-lg-3,
  .py-lg-3 {
    padding-bottom: 1rem !important;
  }
  .pl-lg-3,
  .px-lg-3 {
    padding-left: 1rem !important;
  }
  .p-lg-4 {
    padding: 1.5rem !important;
  }
  .pt-lg-4,
  .py-lg-4 {
    padding-top: 1.5rem !important;
  }
  .pr-lg-4,
  .px-lg-4 {
    padding-right: 1.5rem !important;
  }
  .pb-lg-4,
  .py-lg-4 {
    padding-bottom: 1.5rem !important;
  }
  .pl-lg-4,
  .px-lg-4 {
    padding-left: 1.5rem !important;
  }
  .p-lg-5 {
    padding: 3rem !important;
  }
  .pt-lg-5,
  .py-lg-5 {
    padding-top: 3rem !important;
  }
  .pr-lg-5,
  .px-lg-5 {
    padding-right: 3rem !important;
  }
  .pb-lg-5,
  .py-lg-5 {
    padding-bottom: 3rem !important;
  }
  .pl-lg-5,
  .px-lg-5 {
    padding-left: 3rem !important;
  }
  .m-lg-auto {
    margin: auto !important;
  }
  .mt-lg-auto,
  .my-lg-auto {
    margin-top: auto !important;
  }
  .mr-lg-auto,
  .mx-lg-auto {
    margin-right: auto !important;
  }
  .mb-lg-auto,
  .my-lg-auto {
    margin-bottom: auto !important;
  }
  .ml-lg-auto,
  .mx-lg-auto {
    margin-left: auto !important;
  }
}
@media (min-width: 1200px) {
  .m-xl-0 {
    margin: 0 !important;
  }
  .mt-xl-0,
  .my-xl-0 {
    margin-top: 0 !important;
  }
  .mr-xl-0,
  .mx-xl-0 {
    margin-right: 0 !important;
  }
  .mb-xl-0,
  .my-xl-0 {
    margin-bottom: 0 !important;
  }
  .ml-xl-0,
  .mx-xl-0 {
    margin-left: 0 !important;
  }
  .m-xl-1 {
    margin: 0.25rem !important;
  }
  .mt-xl-1,
  .my-xl-1 {
    margin-top: 0.25rem !important;
  }
  .mr-xl-1,
  .mx-xl-1 {
    margin-right: 0.25rem !important;
  }
  .mb-xl-1,
  .my-xl-1 {
    margin-bottom: 0.25rem !important;
  }
  .ml-xl-1,
  .mx-xl-1 {
    margin-left: 0.25rem !important;
  }
  .m-xl-2 {
    margin: 0.5rem !important;
  }
  .mt-xl-2,
  .my-xl-2 {
    margin-top: 0.5rem !important;
  }
  .mr-xl-2,
  .mx-xl-2 {
    margin-right: 0.5rem !important;
  }
  .mb-xl-2,
  .my-xl-2 {
    margin-bottom: 0.5rem !important;
  }
  .ml-xl-2,
  .mx-xl-2 {
    margin-left: 0.5rem !important;
  }
  .m-xl-3 {
    margin: 1rem !important;
  }
  .mt-xl-3,
  .my-xl-3 {
    margin-top: 1rem !important;
  }
  .mr-xl-3,
  .mx-xl-3 {
    margin-right: 1rem !important;
  }
  .mb-xl-3,
  .my-xl-3 {
    margin-bottom: 1rem !important;
  }
  .ml-xl-3,
  .mx-xl-3 {
    margin-left: 1rem !important;
  }
  .m-xl-4 {
    margin: 1.5rem !important;
  }
  .mt-xl-4,
  .my-xl-4 {
    margin-top: 1.5rem !important;
  }
  .mr-xl-4,
  .mx-xl-4 {
    margin-right: 1.5rem !important;
  }
  .mb-xl-4,
  .my-xl-4 {
    margin-bottom: 1.5rem !important;
  }
  .ml-xl-4,
  .mx-xl-4 {
    margin-left: 1.5rem !important;
  }
  .m-xl-5 {
    margin: 3rem !important;
  }
  .mt-xl-5,
  .my-xl-5 {
    margin-top: 3rem !important;
  }
  .mr-xl-5,
  .mx-xl-5 {
    margin-right: 3rem !important;
  }
  .mb-xl-5,
  .my-xl-5 {
    margin-bottom: 3rem !important;
  }
  .ml-xl-5,
  .mx-xl-5 {
    margin-left: 3rem !important;
  }
  .p-xl-0 {
    padding: 0 !important;
  }
  .pt-xl-0,
  .py-xl-0 {
    padding-top: 0 !important;
  }
  .pr-xl-0,
  .px-xl-0 {
    padding-right: 0 !important;
  }
  .pb-xl-0,
  .py-xl-0 {
    padding-bottom: 0 !important;
  }
  .pl-xl-0,
  .px-xl-0 {
    padding-left: 0 !important;
  }
  .p-xl-1 {
    padding: 0.25rem !important;
  }
  .pt-xl-1,
  .py-xl-1 {
    padding-top: 0.25rem !important;
  }
  .pr-xl-1,
  .px-xl-1 {
    padding-right: 0.25rem !important;
  }
  .pb-xl-1,
  .py-xl-1 {
    padding-bottom: 0.25rem !important;
  }
  .pl-xl-1,
  .px-xl-1 {
    padding-left: 0.25rem !important;
  }
  .p-xl-2 {
    padding: 0.5rem !important;
  }
  .pt-xl-2,
  .py-xl-2 {
    padding-top: 0.5rem !important;
  }
  .pr-xl-2,
  .px-xl-2 {
    padding-right: 0.5rem !important;
  }
  .pb-xl-2,
  .py-xl-2 {
    padding-bottom: 0.5rem !important;
  }
  .pl-xl-2,
  .px-xl-2 {
    padding-left: 0.5rem !important;
  }
  .p-xl-3 {
    padding: 1rem !important;
  }
  .pt-xl-3,
  .py-xl-3 {
    padding-top: 1rem !important;
  }
  .pr-xl-3,
  .px-xl-3 {
    padding-right: 1rem !important;
  }
  .pb-xl-3,
  .py-xl-3 {
    padding-bottom: 1rem !important;
  }
  .pl-xl-3,
  .px-xl-3 {
    padding-left: 1rem !important;
  }
  .p-xl-4 {
    padding: 1.5rem !important;
  }
  .pt-xl-4,
  .py-xl-4 {
    padding-top: 1.5rem !important;
  }
  .pr-xl-4,
  .px-xl-4 {
    padding-right: 1.5rem !important;
  }
  .pb-xl-4,
  .py-xl-4 {
    padding-bottom: 1.5rem !important;
  }
  .pl-xl-4,
  .px-xl-4 {
    padding-left: 1.5rem !important;
  }
  .p-xl-5 {
    padding: 3rem !important;
  }
  .pt-xl-5,
  .py-xl-5 {
    padding-top: 3rem !important;
  }
  .pr-xl-5,
  .px-xl-5 {
    padding-right: 3rem !important;
  }
  .pb-xl-5,
  .py-xl-5 {
    padding-bottom: 3rem !important;
  }
  .pl-xl-5,
  .px-xl-5 {
    padding-left: 3rem !important;
  }
  .m-xl-auto {
    margin: auto !important;
  }
  .mt-xl-auto,
  .my-xl-auto {
    margin-top: auto !important;
  }
  .mr-xl-auto,
  .mx-xl-auto {
    margin-right: auto !important;
  }
  .mb-xl-auto,
  .my-xl-auto {
    margin-bottom: auto !important;
  }
  .ml-xl-auto,
  .mx-xl-auto {
    margin-left: auto !important;
  }
}
.text-justify {
  text-align: justify !important;
}
.text-nowrap {
  white-space: nowrap !important;
}
.text-truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.text-left {
  text-align: left !important;
}
.text-right {
  text-align: right !important;
}
.text-center {
  text-align: center !important;
}
@media (min-width: 576px) {
  .text-sm-left {
    text-align: left !important;
  }
  .text-sm-right {
    text-align: right !important;
  }
  .text-sm-center {
    text-align: center !important;
  }
}
@media (min-width: 768px) {
  .text-md-left {
    text-align: left !important;
  }
  .text-md-right {
    text-align: right !important;
  }
  .text-md-center {
    text-align: center !important;
  }
}
@media (min-width: 992px) {
  .text-lg-left {
    text-align: left !important;
  }
  .text-lg-right {
    text-align: right !important;
  }
  .text-lg-center {
    text-align: center !important;
  }
}
@media (min-width: 1200px) {
  .text-xl-left {
    text-align: left !important;
  }
  .text-xl-right {
    text-align: right !important;
  }
  .text-xl-center {
    text-align: center !important;
  }
}
.text-lowercase {
  text-transform: lowercase !important;
}
.text-uppercase {
  text-transform: uppercase !important;
}
.text-capitalize {
  text-transform: capitalize !important;
}
.font-weight-light {
  font-weight: 300 !important;
}
.font-weight-normal {
  font-weight: 400 !important;
}
.font-weight-bold {
  font-weight: 700 !important;
}
.font-italic {
  font-style: italic !important;
}
.text-white {
  color: #fff !important;
}
.text-primary {
  color: #007bff !important;
}
a.text-primary:focus, a.text-primary:hover {
  color: #0062cc !important;
}
.text-secondary {
  color: #868e96 !important;
}
a.text-secondary:focus, a.text-secondary:hover {
  color: #6c757d !important;
}
.text-success {
  color: #28a745 !important;
}
a.text-success:focus, a.text-success:hover {
  color: #1e7e34 !important;
}
.text-info {
  color: #17a2b8 !important;
}
a.text-info:focus, a.text-info:hover {
  color: #117a8b !important;
}
.text-warning {
  color: #ffc107 !important;
}
a.text-warning:focus, a.text-warning:hover {
  color: #d39e00 !important;
}
.text-danger {
  color: #dc3545 !important;
}
a.text-danger:focus, a.text-danger:hover {
  color: #bd2130 !important;
}
.text-light {
  color: #f8f9fa !important;
}
a.text-light:focus, a.text-light:hover {
  color: #dae0e5 !important;
}
.text-dark {
  color: #343a40 !important;
}
a.text-dark:focus, a.text-dark:hover {
  color: #1d2124 !important;
}
.text-muted {
  color: #868e96 !important;
}
.text-hide {
  font: 0/0 a;
  color: transparent;
  text-shadow: none;
  background-color: transparent;
  border: 0;
}
.visible {
  visibility: visible !important;
}
.invisible {
  visibility: hidden !important;
}
/*# sourceMappingURL=bootstrap.css.map */#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}

  

    
  SE 234 Project
  This is the mock app for the SE 234 project
  
  
    
      
        
          Products
          
        
        
          Carts
            
          
        
        
      
      Logout
    
  

  

  
      Well done! You successfully added the transaction.
    
  
  
    Products
    
    
  
  
    
      
        
          

            
          
          
            Garden

            The garden which you can grow everything on earth
            
              Price is 20,000.00 THB
            
            
               add to cart
            
            
          
        
      
        
          

            
          
          
            Banana

            A good fruit with very cheap price
            
              Price is 150.00 THB
            
            
               add to cart
            
            
          
        
      
        
          

            
          
          
            Orange

            Nothing good about it
            
              Price is 280.00 THB
            
            
               add to cart
            
            
          
        
      
        
          

            
          
          
            Papaya

            Use for papaya salad
            
              Price is 12.00 THB
            
            
               add to cart
            
            
          
        
      
        
          

            
          
          
            Rambutan

            An expensive fruit from the sout
            
              Price is 20.00 THB
            
            
               add to cart
            
            
          
        
      
    
  






/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
