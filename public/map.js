function removeMap(map) {
    map.remove();
    delete map;
}

function bindPopup(circle, image, map) {
    circle.bindPopup(() => renderPopup(image, map));
}

function renderPopup(image, map) {
    const {id, sha256, urlSmall, urlLarge, timestamp} = image;
    const date = new Date(timestamp).toDateString();
    const location = formatLocation(image);
    const outer = document.createElement('div');
    const catImage = makeImageLink(urlLarge, urlSmall, `photo #${id}, showing one or more cats`);
    outer.appendChild(catImage);

    const footer = document.createElement('div');
    footer.className = 'popup-footer';

    const description = document.createElement('div');
    description.innerText = `Photo #${id}. Taken on ${date} in ${location}`;
    footer.appendChild(description);

    const favButton = makeFavoriteButton(sha256);
    favButton.mount(footer);

    if (navigator.share) {
        const shareButton = makeIconButton(
            'static/share.svg',
            'share',
            () => shareCatto(id, map.getZoom())
        );

        shareButton.mount(footer);
    }

    outer.appendChild(footer);
    return outer;

}

function makeImageLink(href, src, alt) {
    const img = document.createElement('img');
    img.src = src;
    img.alt = alt;

    const a = document.createElement('a');
    a.href = href;
    a.appendChild(img);
    return a;
}

function makeFavoriteButton(imageHash) {
    const favs = new Favorites();
    const icon = favs.iconForStatus(imageHash);
    const alt = favs.has(imageHash) ? 'remove this cat from your favorites' : 'add this cat to your favorites';
    const favButton = makeIconButton(icon, alt);

    favButton.setOnClick(() => {
        favs.toggle(imageHash);
        favButton.setIcon(favs.iconForStatus(imageHash));
    });

    return favButton;
}

function makeIconButton(icon, alt, onClick) {
    const ib = {
        img: document.createElement('img'),
        button: document.createElement('button'),
        setIcon: (icon) => ib.img.src = icon,
        setOnClick: (handler) => ib.button.onclick = handler,
        mount: (container) => container.appendChild(ib.button)
    };

    ib.img.className = 'icon';
    ib.img.src = icon;
    ib.img.alt = alt;

    ib.button.appendChild(ib.img);

    if (onClick) {
        ib.setOnClick(onClick);
    }

    return ib;
}

function shareCatto(imageId, zoomLevel) {
    const protocol = window.location.hostname === 'localhost' ? 'http' : 'https';
    const url = `${protocol}://${window.location.hostname}${window.location.pathname}?imageId=${imageId}&zoomLevel=${zoomLevel}`;

    navigator.share({
        title: `${document.title} #${imageId}`,
        text: 'Check out this cat!',
        url: url,
    })
        .then(() => console.log('catto sharing is catto caring'))
        .catch(error => console.log('error sharing:', error));
}

function formatLocation(image) {
    const {city, country} = image;
    return city ? `${city}, ${country}` : country
}

function Favorites() {
    const load = () => new Set(JSON.parse(localStorage.getItem('favorites')));
    const store = (s) => localStorage.setItem('favorites', JSON.stringify([...s]));

    return {
        has: (imageHash) => load().has(imageHash),
        read: () => [...load()],
        iconForStatus: (imageHash) => load().has(imageHash) ? 'favorite-filled.svg' : 'favorite.svg',

        toggle: (imageHash) => {
            let s = load();
            if (s.has(imageHash)) {
                s.delete(imageHash);
            } else {
                s.add(`${imageHash}`);
            }
            store(s);
        },
    }
}

// This is the implementation for the PopupTemplate component, which for some reason only
// works "once".
//function renderPopup(image, map) {
//    const {id, urlSmall, urlLarge, timestamp} = image;
//    const date = new Date(timestamp).toDateString();
//    const location = formatLocation(image);
//    const template = document.querySelector('#popup');
//    const clone = template.content.cloneNode(true);
//
//    const link = clone.querySelector('#link');
//    link.id = `link-${image.id}`;
//    link.href = urlLarge;
//
//    const img = clone.querySelector('#catto');
//    img.id = `catto-${image.id}`
//    img.src = urlSmall;
//
//    const description = clone.querySelector('#description');
//    description.id = `description-${image.id}`;
//    description.innerText = `Photo #${id}. Taken on ${date} in ${location}`;
//
//    renderFavoriteButton(clone, image.sha256);
//    renderShareButton(clone, image.id, map);
//    return clone;
//}

//function renderFavoriteButton(popup, imageHash) {
//    const icon = popup.querySelector('#icon');
//    icon.id = `icon-${imageHash}`;
//    icon.src = iconForFavStatus(imageHash);
//
//    const alt = hasFavorite(imageHash) ? 'remove this cat from your favorites' : 'add this cat to your favorites';
//    icon.setAttribute('alt', alt);
//
//    const fav = popup.querySelector('#favorite')
//    fav.id = `favorite-${imageHash}`;
//
//    fav.onclick = () => {
//        toggleFavorite(imageHash);
//        icon.src = iconForFavStatus(imageHash);
//    };
//}

//function renderShareButton(popup, imageId, map) {
//    const share = popup.querySelector('#share');
//    share.id = `share-${imageId}`;
//
//    if (navigator.share) {
//        share.removeAttribute("hidden");
//        share.onclick = () => shareCatto(imageId, map.getZoom());
//    }
//}
