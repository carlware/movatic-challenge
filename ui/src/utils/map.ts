const { REACT_APP_MAPBOX_ACCESS_TOKEN } = process.env;

export const getStaticMapURL = (lat?: number, lng?: number) => {
	if (lat === undefined || lng === undefined) {
		return undefined;
	}
	return `https://api.mapbox.com/styles/v1/mapbox/streets-v12/static/pin-s+000(${lng},${lat})/${lng},${lat},13,0/500x300?access_token=${REACT_APP_MAPBOX_ACCESS_TOKEN}`;
};
