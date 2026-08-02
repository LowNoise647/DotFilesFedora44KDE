"use strict";

var blacklist = [
    "ksmserver ksmserver",
    "ksmserver-logout-greeter ksmserver-logout-greeter",
    "kscreenlocker_greet kscreenlocker_greet",
    "ksplashqml ksplashqml"
];

function isNormalWindow(window) {
    if (blacklist.indexOf(window.windowClass) !== -1) {
        return false;
    }

    if (!window.managed) {
        return false;
    }

    if (window.popupWindow ||
        window.dock ||
        window.splash ||
        window.toolbar ||
        window.notification ||
        window.onScreenDisplay ||
        window.criticalNotification ||
        window.appletPopup) {
        return false;
    }

    return true;
}

function setWindowVisualRoles(window, enabled) {
    window.setData(Effect.WindowForceBlurRole, enabled);
    window.setData(Effect.WindowForceBackgroundContrastRole, enabled);
}

var bounceWindowsEffect = {
    openDuration: animationTime(250),
    closeDuration: animationTime(200),

    slotWindowAdded: function (window) {
        if (effects.hasActiveFullScreenEffect) {
            return;
        }

        if (!isNormalWindow(window) || !window.visible) {
            return;
        }

        if (!effect.grab(window, Effect.WindowAddedGrabRole)) {
            return;
        }

        setWindowVisualRoles(window, true);

        window.scaleInAnimation = animate({
            window: window,
            curve: QEasingCurve.OutBack,
            duration: bounceWindowsEffect.openDuration,
            type: Effect.Scale,
            from: 0.8,
            to: 1.0
        });

        window.opacityInAnimation = animate({
            window: window,
            curve: QEasingCurve.OutCubic,
            duration: bounceWindowsEffect.openDuration,
            type: Effect.Opacity,
            from: 0.0,
            to: 1.0
        });
    },

    slotWindowClosed: function (window) {
        if (effects.hasActiveFullScreenEffect) {
            return;
        }

        if (!isNormalWindow(window) || !window.visible || window.skipsCloseAnimation) {
            return;
        }

        if (!effect.grab(window, Effect.WindowClosedGrabRole)) {
            return;
        }

        setWindowVisualRoles(window, true);

        window.scaleOutAnimation = animate({
            window: window,
            curve: QEasingCurve.InBack,
            duration: bounceWindowsEffect.closeDuration,
            type: Effect.Scale,
            from: 1.0,
            to: 0.8
        });

        window.opacityOutAnimation = animate({
            window: window,
            curve: QEasingCurve.InCubic,
            duration: bounceWindowsEffect.closeDuration,
            type: Effect.Opacity,
            from: 1.0,
            to: 0.0
        });
    },

    slotWindowDataChanged: function (window, role) {
        if (role === Effect.WindowAddedGrabRole) {
            if (window.scaleInAnimation) cancel(window.scaleInAnimation);
            if (window.opacityInAnimation) cancel(window.opacityInAnimation);
        } else if (role === Effect.WindowClosedGrabRole) {
            if (window.scaleOutAnimation) cancel(window.scaleOutAnimation);
            if (window.opacityOutAnimation) cancel(window.opacityOutAnimation);
        }
    },

    init: function () {
        effects.windowAdded.connect(this.slotWindowAdded);
        effects.windowClosed.connect(this.slotWindowClosed);
        effects.windowDataChanged.connect(this.slotWindowDataChanged);
    }
};

bounceWindowsEffect.init();