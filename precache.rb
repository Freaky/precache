#!/usr/bin/env ruby
# frozen_string_literal: true

require 'find'

threads = 8
queue = SizedQueue.new(1024)
readers = Array.new(threads) do
  Thread.new do
    nullwriter = File.new('/dev/null', 'w')
    while (fn = queue.deq)
      begin
        File.open(fn) { |f| IO.copy_stream(f, nullwriter) }
      rescue
      end
    end
  end
end

Find.find(*ARGV) { |fn| queue.enq(fn) }

queue.close
readers.each(&:join)
