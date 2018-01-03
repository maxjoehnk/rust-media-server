import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
    name: 'duration'
})
export class DurationPipe implements PipeTransform {

    transform(value: any, args?: any): any {
        const minutes = Math.floor(value / 60);
        const secs = value % 60;
        return `${minutes}:${this.pad(secs)}`;
    }

    private pad(number: number): string {
        return `0${number}`.slice(-2);
    }
}
