import axios, { type AxiosResponse } from "axios";
const { REACT_APP_API_URL } = process.env;

export interface StationInformation {
	lon: number;
	lat: number;
	rental_uris: RentalUris;
	_bcycle_station_type: string;
	region_id: string;
	address: string;
	name: string;
	station_id: string;
}

export interface RentalUris {
	ios: string;
	android: string;
}

export interface StationInformationResponse {
	last_updated: number;
	data: {
		stations: StationInformation[];
	};
	version: string;
}

export const getStationsInformation = async () => {
	const resp: AxiosResponse<StationInformationResponse> = await axios.get(
		`${REACT_APP_API_URL}/station_information`,
	);
	return resp.data.data.stations;
};
