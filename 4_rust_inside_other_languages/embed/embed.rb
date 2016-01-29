require 'ffi'
start_time = Time.now
module Hello
  extend FFI::Library
  ffi_lib 'target/release/libembed.so'
  attach_function :process, [], :void
end

Hello.process

end_time= Time.now

puts 'done!'

puts end_time - start_time
