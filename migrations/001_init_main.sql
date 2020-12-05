create table main (
	key text primary key,
	val jsonb not null
);

insert into main (key, val)
values ('config', '{"captcha":false,"mature":false,"prune_threads":false,"thread_expiry":7,"max_size":5,"default_lang":"en_GB","default_theme":"moe","image_root_override":"","links":{"4chan":"http://www.4chan.org/"},"email_errors":false,"JPEG_thumbnails":false,"disable_robots":false,"max_width":6000,"max_height":6000,"email_errors_server_port":587,"char_score":170,"post_creation_score":15000,"image_score":15000,"root_URL":"http://127.0.0.1","salt":"LALALALALALALALALALALALALALALALALALALALA","email_errors_address":"","email_errors_password":"","email_errors_server_address":"","feedback_email":"","FAQ":"Supported upload file types are JPEG, PNG, APNG, WEBM, MP3, FLAC, MP4, OGG, PDF, ZIP, 7Z, TAR.GZ, TAR.XZ, RAR, CBZ, CBR.\n\u003chr\u003eEncase text in:\n  ** for spoilers\n  @@ for bold\n  ~~ for italics\n  ^r for red text\n  ^b for blue text\n  `` for programing code highlighting\n\u003chr\u003eHash commands:\n#d100 #2d100 - Roll dice\n#flip - Coin flip\n#8ball - An 8ball\n#sw24:30 #sw2:24:30 #sw24:30+30 #sw24:30-30 - \"Syncwatch\" synchronized time counter","captcha_tags":["patchouli_knowledge","cirno","hakurei_reimu"]}');

create or replace function notify_config_updates()
returns trigger
language plpgsql
as $$
begin
	if new.key == 'config' then
		perform pg_notify('config.updated', '');
	end if;
	return new;
end;
$$;

create trigger notify_config_updates
after update on main
for each row
execute function notify_config_updates();
