import { Track } from '../library/album.model';

export interface Playlist {
    id?: number;
    title: string;
    tracks: Track[];
}