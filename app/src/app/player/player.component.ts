import { Component, OnInit } from '@angular/core';
import { PlayerService, PlayerState } from './player.service';
import { Observable } from 'rxjs/Observable';
import { Track } from '../library/album.model';
import { ObservableMedia } from '@angular/flex-layout';
import 'rxjs/add/observable/interval';
import 'rxjs/add/operator/switchMap';

@Component({
    selector: 'rms-player',
    templateUrl: './player.component.html',
    styleUrls: ['./player.component.scss']
})
export class PlayerComponent implements OnInit {

    playing = false;
    current: Track;

    constructor(private player: PlayerService,
                private media: ObservableMedia) {
    }

    ngOnInit() {
        Observable
            .interval(1000)
            .switchMap(this.player.getState.bind(this.player))
            .subscribe((state: PlayerState) => {
                this.playing = state.playing;
                this.current = state.current;
            });
    }

    toggle() {
        let observable;
        if (this.playing) {
            observable = this.player.pause();
        } else {
            observable = this.player.play();
        }
        observable.subscribe(() => {
            this.playing = !this.playing;
        });
    }

    next() {
        this.player
            .next()
            .subscribe(() => {
            });
    }

    prev() {
        this.player
            .prev()
            .subscribe(() => {
            });
    }

    openQueue() {
    }

    openBottomQueue() {
        if (this.media.isActive('gt-sm')) {
            return;
        }
    }
}
